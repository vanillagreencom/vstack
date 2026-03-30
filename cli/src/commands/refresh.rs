use crate::config::{self, ItemKind};
use crate::harness::Harness;
use anyhow::Result;

/// Regenerate all installed agent files using current vstack.toml customizations.
pub fn run(global: bool) -> Result<()> {
    let lock_path = config::lock_file_path(global);
    let lock = config::LockFile::load(&lock_path)?;
    let project_root = config::project_root();

    let mut project_config = crate::mapping::ProjectConfig::load(&project_root);

    let agent_entries: Vec<_> = lock
        .entries
        .iter()
        .filter(|(_, e)| e.kind == ItemKind::Agent)
        .collect();

    if agent_entries.is_empty() {
        eprintln!("No agents installed. Run `vstack add` first.");
        return Ok(());
    }

    let installed_skills: Vec<String> = lock
        .entries
        .iter()
        .filter(|(_, e)| e.kind == ItemKind::Skill)
        .map(|(name, _)| name.clone())
        .collect();

    // Find the source directory from any installed entry
    let source_dir = lock
        .entries
        .values()
        .find_map(|e| {
            let p = std::path::Path::new(&e.source);
            if p.is_dir() {
                Some(p.to_path_buf())
            } else {
                // Try cached clone
                let cache = config::global_state_dir().join(".vstack").join("cache");
                let key = e.source.replace('/', "_");
                let cached = cache.join(&key);
                cached.is_dir().then_some(cached)
            }
        })
        .unwrap_or_else(|| project_root.clone());

    let mapping = crate::mapping::MappingConfig::load(&source_dir);
    let source_agents = crate::agent::discover_agents(&source_dir.join("agents")).unwrap_or_default();
    let source_skills = crate::skill::discover_skills(&source_dir.join("skills")).unwrap_or_default();
    let source_hooks = crate::hook::discover_hooks(&source_dir.join("hooks")).unwrap_or_default();

    let mut refreshed = 0usize;

    for (name, entry) in &agent_entries {
        let Some(agent) = source_agents.iter().find(|a| &a.name == *name) else {
            continue;
        };

        let matched_skill_names =
            mapping.skills_for_agent(&agent.name, &agent.role, &installed_skills);
        let mut skill_pairs: Vec<(String, String)> = matched_skill_names
            .iter()
            .filter_map(|sname| {
                source_skills
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
            .hooks_for_agent(&agent.role, &source_hooks)
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

        // Build extras (toml takes precedence, file-extracted is fallback via save_extracted above)
        let extras = crate::agent::AgentExtras {
            guidance: project_config.guidance_for(&agent.name).map(String::from),
            instructions: project_config
                .instructions_for(&agent.name)
                .map(String::from),
        };

        for harness_id in &entry.harnesses {
            if let Some(harness) = Harness::from_id(harness_id) {
                let _ = harness.generate_agent(agent, global, &skill_pairs, &matched_hooks, &extras);
                refreshed += 1;
            }
        }
    }

    eprintln!(
        "Refreshed {} agent file(s) from vstack.toml",
        refreshed
    );
    Ok(())
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
