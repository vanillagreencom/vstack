use crate::config::{self, ItemKind};
use crate::harness::Harness;
use crate::installer;
use anyhow::Result;
use std::path::PathBuf;

/// Regenerate all installed agent files and re-copy skills from source.
pub fn run(global: bool) -> Result<()> {
    let lock_path = config::lock_file_path(global);
    let lock = config::LockFile::load(&lock_path)?;
    let project_root = config::project_root();

    if lock.entries.is_empty() {
        eprintln!("Nothing installed. Run `vstack add` first.");
        return Ok(());
    }

    if !global {
        crate::mapping::ensure_project_config(&project_root);
    }
    let mut project_config = crate::mapping::ProjectConfig::load(&project_root);

    // Resolve source directories from lock file entries
    let source_dirs = resolve_sources(&lock);
    if source_dirs.is_empty() {
        eprintln!("Could not locate any package sources. Run `vstack add` to reinstall.");
        return Ok(());
    }

    // Aggregate source data from all resolved sources
    let mut all_source_agents = Vec::new();
    let mut all_source_skills = Vec::new();
    let mut all_source_hooks = Vec::new();
    let mut mapping = crate::mapping::MappingConfig::default();

    for dir in &source_dirs {
        mapping = crate::mapping::MappingConfig::load(dir);
        all_source_agents.extend(
            crate::agent::discover_agents(&dir.join("agents")).unwrap_or_default(),
        );
        all_source_skills.extend(
            crate::skill::discover_skills(&dir.join("skills")).unwrap_or_default(),
        );
        all_source_hooks.extend(
            crate::hook::discover_hooks(&dir.join("hooks")).unwrap_or_default(),
        );
    }

    let installed_skills: Vec<String> = lock
        .entries
        .iter()
        .filter(|(_, e)| e.kind == ItemKind::Skill)
        .map(|(name, _)| name.clone())
        .collect();

    // Refresh agents
    let mut agents_refreshed = 0usize;
    let agent_entries: Vec<_> = lock
        .entries
        .iter()
        .filter(|(_, e)| e.kind == ItemKind::Agent)
        .collect();

    for (name, entry) in &agent_entries {
        let Some(agent) = all_source_agents.iter().find(|a| &a.name == *name) else {
            eprintln!("  ! {} — source not found, skipped", name);
            continue;
        };

        let matched_skill_names =
            mapping.skills_for_agent(&agent.name, &agent.role, &installed_skills);
        let mut skill_pairs: Vec<(String, String)> = matched_skill_names
            .iter()
            .filter_map(|sname| {
                all_source_skills
                    .iter()
                    .find(|s| &s.name == sname)
                    .map(|s| (s.name.clone(), s.description.clone()))
            })
            .collect();

        for cs in project_config.custom_skills_for(&agent.name) {
            if !skill_pairs.iter().any(|(n, _)| *n == cs.0) {
                skill_pairs.push(cs);
            }
        }

        let matched_hooks: Vec<crate::hook::Hook> = mapping
            .hooks_for_agent(&agent.role, &all_source_hooks)
            .into_iter()
            .cloned()
            .collect();

        // Extract user sections from existing files before overwriting
        for harness_id in &entry.harnesses {
            if let Some(harness) = Harness::from_id(harness_id) {
                let existing_path = harness
                    .agents_dir(global)
                    .join(harness.agent_filename(&agent.name));
                let file_extras = read_existing_extras(&existing_path, harness);
                project_config.save_extracted(&project_root, &agent.name, &file_extras);
            }
        }

        let extras = crate::agent::AgentExtras {
            guidance: project_config.guidance_for(&agent.name).map(String::from),
            instructions: project_config
                .instructions_for(&agent.name)
                .map(String::from),
        };

        for harness_id in &entry.harnesses {
            if let Some(harness) = Harness::from_id(harness_id) {
                let _ =
                    harness.generate_agent(agent, global, &skill_pairs, &matched_hooks, &extras);
                agents_refreshed += 1;
            }
        }
    }

    // Refresh skills — re-copy from source
    let mut skills_refreshed = 0usize;
    let skill_entries: Vec<_> = lock
        .entries
        .iter()
        .filter(|(_, e)| e.kind == ItemKind::Skill)
        .collect();

    for (name, entry) in &skill_entries {
        let Some(skill) = all_source_skills.iter().find(|s| &s.name == *name) else {
            continue;
        };

        for harness_id in &entry.harnesses {
            if let Some(harness) = Harness::from_id(harness_id) {
                let _ = installer::install_skill(skill, harness, global, entry.method);
                skills_refreshed += 1;
            }
        }
    }

    eprintln!(
        "Refreshed {} agent(s), {} skill(s)",
        agents_refreshed, skills_refreshed
    );
    Ok(())
}

/// Resolve source directories from lock file entries.
/// Handles local paths, "." (walks up from CWD), and remote shorthand (cached clones).
fn resolve_sources(lock: &config::LockFile) -> Vec<PathBuf> {
    let mut sources: Vec<PathBuf> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for entry in lock.entries.values() {
        if seen.contains(&entry.source) {
            continue;
        }
        seen.insert(entry.source.clone());

        if let Some(dir) = resolve_single_source(&entry.source) {
            if !sources.contains(&dir) {
                sources.push(dir);
            }
        }
    }

    // Fallback: walk up from CWD to find a vstack source repo
    if sources.is_empty() {
        if let Ok(mut dir) = std::env::current_dir() {
            loop {
                if is_vstack_source(&dir) {
                    sources.push(dir);
                    break;
                }
                if !dir.pop() {
                    break;
                }
            }
        }
    }

    // Fallback: try the source registry (cached remote repos)
    if sources.is_empty() {
        let reg_path = config::source_registry_path();
        if let Ok(registry) = config::SourceRegistry::load(&reg_path) {
            for entry in registry
                .current
                .iter()
                .chain(registry.entries.iter())
            {
                if let Some(dir) = resolve_single_source(entry) {
                    if !sources.contains(&dir) {
                        sources.push(dir);
                    }
                }
            }
        }
    }

    sources
}

fn resolve_single_source(source: &str) -> Option<PathBuf> {
    // Absolute or relative path that exists
    let p = std::path::Path::new(source);
    if p.is_absolute() && p.is_dir() && is_vstack_source(p) {
        return Some(p.to_path_buf());
    }

    // "." — walk up from CWD
    if source == "." {
        let mut dir = std::env::current_dir().ok()?;
        loop {
            if is_vstack_source(&dir) {
                return Some(dir);
            }
            if !dir.pop() {
                break;
            }
        }
        return None;
    }

    // Remote shorthand (owner/repo) — check cached clone
    let cache_dir = config::global_base_dir()
        .join(".vstack")
        .join("cache");
    let key = source.replace('/', "_");
    let cached = cache_dir.join(&key);
    if cached.is_dir() {
        return Some(cached);
    }

    None
}

fn is_vstack_source(dir: &std::path::Path) -> bool {
    if dir
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with('.'))
    {
        return false;
    }
    if dir.join("agents").is_dir() || dir.join("skills").is_dir() {
        let count = [
            dir.join("agents").is_dir(),
            dir.join("skills").is_dir(),
            dir.join("hooks").is_dir(),
        ]
        .iter()
        .filter(|&&b| b)
        .count();
        return count >= 2;
    }
    false
}

fn read_existing_extras(
    path: &std::path::Path,
    harness: Harness,
) -> crate::agent::AgentExtras {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Default::default();
    };
    let body = if matches!(harness, Harness::Codex) {
        crate::agent::extract_body_from_codex_toml(&content).unwrap_or(content)
    } else {
        content
    };
    crate::agent::extract_user_sections(&body)
}
