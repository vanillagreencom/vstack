use crate::agent;
use crate::config::{self, InstallMethod, LockFile};
use crate::harness::Harness;
use crate::hook;
use crate::installer;
use crate::skill;
use crate::tui;
use anyhow::Context;
use anyhow::Result;
use std::path::{Path, PathBuf};

struct ResolvedSource {
    source: String,
    label: String,
    dir: PathBuf,
    persist: bool,
}

fn source_label(source: &str) -> String {
    if Path::new(source).exists() {
        let path = Path::new(source);
        return path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(source)
            .to_string();
    }

    let trimmed = source
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .trim_start_matches("https://github.com/")
        .trim_start_matches("http://github.com/")
        .trim_start_matches("git@github.com:");
    trimmed.to_string()
}

fn build_source_options(
    registry: &config::SourceRegistry,
    resolved: &ResolvedSource,
) -> Vec<tui::RepoOption> {
    let mut sources = Vec::new();
    sources.push(crate::REPO.to_string());
    if let Some(current) = &registry.current {
        sources.push(current.clone());
    }
    sources.extend(registry.entries.iter().cloned());
    if !sources.iter().any(|source| source == &resolved.source) {
        sources.push(resolved.source.clone());
    }

    let mut options = Vec::new();
    for source in sources {
        if options
            .iter()
            .any(|option: &tui::RepoOption| option.source == source)
        {
            continue;
        }
        options.push(tui::RepoOption {
            label: source_label(&source),
            source,
        });
    }
    options
}

fn resolve_source_for_app(
    source: Option<&str>,
    registry: &config::SourceRegistry,
) -> Result<ResolvedSource> {
    match source {
        Some(path) if Path::new(path).exists() => {
            let dir = std::fs::canonicalize(path)?;
            Ok(ResolvedSource {
                source: dir.display().to_string(),
                label: source_label(path),
                dir,
                persist: true,
            })
        }
        Some(source) => Ok(ResolvedSource {
            source: source.to_string(),
            label: source_label(source),
            dir: resolve_source(Some(source))?,
            persist: true,
        }),
        None => {
            let mut dir = std::env::current_dir()?;
            loop {
                if is_vstack_root(&dir) {
                    return Ok(ResolvedSource {
                        source: dir.display().to_string(),
                        label: source_label(dir.to_str().unwrap_or("local")),
                        dir,
                        persist: false,
                    });
                }
                if !dir.pop() {
                    break;
                }
            }

            let source = registry
                .current
                .clone()
                .unwrap_or_else(|| crate::REPO.to_string());
            Ok(ResolvedSource {
                label: source_label(&source),
                dir: resolve_source(Some(&source))?,
                source,
                persist: true,
            })
        }
    }
}

pub fn run(
    source: Option<String>,
    global: bool,
    harness_filter: Option<Vec<String>>,
    skill_filter: Option<Vec<String>>,
    copy: bool,
    yes: bool,
    all: bool,
) -> Result<()> {
    let mut registry =
        config::SourceRegistry::load(&config::source_registry_path()).unwrap_or_default();
    let mut current_source = source.clone();
    let (
        resolved_source,
        selected_agents,
        selected_skills,
        selected_hooks,
        harnesses,
        skipped_harnesses,
        global,
        method,
        update_cli,
    ) = loop {
        let resolved = resolve_source_for_app(current_source.as_deref(), &registry)?;
        if resolved.persist {
            registry.remember(&resolved.source);
            registry.save(&config::source_registry_path())?;
        }
        let source_dir = resolved.dir.clone();
        let agents_dir = source_dir.join("agents");
        let skills_dir = source_dir.join("skills");
        let hooks_dir = source_dir.join("hooks");

        let agents = agent::discover_agents(&agents_dir)?;
        let all_skills = skill::discover_skills(&skills_dir)?;
        let hooks = hook::discover_hooks(&hooks_dir)?;
        let dep_graph = skill::build_dependency_graph(&all_skills);

        let skills = if let Some(ref filter) = skill_filter {
            if filter.iter().any(|f| f == "*") {
                all_skills
            } else {
                let (expanded, auto_added) = skill::expand_dependencies(filter, &dep_graph);
                if !auto_added.is_empty() {
                    eprintln!("Auto-added dependencies: {}", auto_added.join(", "));
                }
                all_skills
                    .into_iter()
                    .filter(|s| expanded.contains(&s.name))
                    .collect()
            }
        } else {
            all_skills
        };

        let total = agents.len() + skills.len() + hooks.len();
        if total == 0 {
            eprintln!(
                "No agents, skills, or hooks found in {}",
                source_dir.display()
            );
            return Ok(());
        }

        eprintln!(
            "Found {} agent(s), {} skill(s), {} hook(s) in {}",
            agents.len(),
            skills.len(),
            hooks.len(),
            source_dir.display()
        );

        if all {
            break (
                resolved,
                agents,
                skills,
                hooks,
                Harness::ALL.to_vec(),
                Vec::new(),
                global,
                if copy {
                    InstallMethod::Copy
                } else {
                    InstallMethod::Symlink
                },
                false,
            );
        } else if yes || harness_filter.is_some() {
            let harnesses = if let Some(ref filter) = harness_filter {
                filter.iter().filter_map(|f| Harness::from_id(f)).collect()
            } else {
                Harness::ALL
                    .iter()
                    .copied()
                    .filter(|h| h.is_detected())
                    .collect::<Vec<_>>()
            };

            if harnesses.is_empty() {
                eprintln!("No harnesses selected or detected. Use --agent to specify.");
                return Ok(());
            }

            break (
                resolved,
                agents,
                skills,
                hooks,
                harnesses,
                Vec::new(),
                global,
                if copy {
                    InstallMethod::Copy
                } else {
                    InstallMethod::Symlink
                },
                false,
            );
        } else {
            let selector = tui::SourceSelectorData {
                current_label: resolved.label.clone(),
                options: build_source_options(&registry, &resolved),
            };
            let items = tui::DiscoveredItems {
                agents,
                skills,
                hooks,
            };
            match tui::run_install_flow(items, &selector)? {
                tui::InstallFlowResult::Install(sel) => {
                    break (
                        resolved,
                        sel.agents,
                        sel.skills,
                        sel.hooks,
                        sel.harnesses,
                        sel.skipped_harnesses,
                        sel.global,
                        sel.method,
                        sel.update_cli,
                    );
                }
                tui::InstallFlowResult::Cancelled => {
                    eprintln!("Installation cancelled.");
                    return Ok(());
                }
                tui::InstallFlowResult::SwitchSource(source) => {
                    current_source = Some(source);
                }
            }
        }
    };

    let source_dir = resolved_source.dir.clone();
    let mapping = crate::mapping::MappingConfig::load(&source_dir);

    if global {
        let unsupported: Vec<Harness> = harnesses
            .iter()
            .copied()
            .filter(|h| !h.supports_global_scope())
            .collect();
        if !unsupported.is_empty() && unsupported.len() == harnesses.len() {
            eprintln!(
                "Global install is not supported for: {}. Rerun from the target project directory for project-scoped install.",
                unsupported
                    .iter()
                    .map(|h| h.name())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            return Ok(());
        }
    }

    let mut harnesses = harnesses;
    let mut skipped_harnesses = skipped_harnesses;
    if global {
        let mut unsupported: Vec<String> = harnesses
            .iter()
            .filter(|h| !h.supports_global_scope())
            .map(|h| h.name().to_string())
            .collect();
        harnesses.retain(|h| h.supports_global_scope());
        skipped_harnesses.append(&mut unsupported);
        skipped_harnesses.sort();
        skipped_harnesses.dedup();

        if !skipped_harnesses.is_empty() {
            eprintln!(
                "Skipping project-only harnesses for global install: {}. Rerun from the target project directory to install those.",
                skipped_harnesses.join(", ")
            );
        }
    }

    // Track what's already installed (to distinguish updates from new installs)
    let pre_lock = config::LockFile::load(&config::lock_file_path(global)).unwrap_or_default();
    let previously_installed: std::collections::HashSet<String> =
        pre_lock.entries.keys().cloned().collect();

    // Perform installation
    let mut results = Vec::new();
    let mut log_lines: Vec<String> = Vec::new();

    for harness in &harnesses {
        for a in &selected_agents {
            // Compute matched skills for this agent
            let available_skill_names: Vec<String> =
                selected_skills.iter().map(|s| s.name.clone()).collect();
            let matched = mapping.skills_for_agent(&a.name, &a.role, &available_skill_names);
            let skill_pairs: Vec<(String, String)> = matched
                .iter()
                .filter_map(|name| {
                    selected_skills
                        .iter()
                        .find(|s| &s.name == name)
                        .map(|s| (s.name.clone(), s.description.clone()))
                })
                .collect();

            // Compute matched hooks
            let matched_hooks: Vec<hook::Hook> = mapping
                .hooks_for_agent(&a.role, &selected_hooks)
                .into_iter()
                .cloned()
                .collect();

            let result =
                installer::install_agent(a, *harness, global, &skill_pairs, &matched_hooks)?;
            log_lines.push(result.detail.clone());
            results.push(result);
        }

        for s in &selected_skills {
            let result = installer::install_skill(s, *harness, global, method)?;
            log_lines.push(result.detail.clone());
            results.push(result);
        }

        for h in &selected_hooks {
            let detail = installer::install_hook(h, *harness, global, &selected_agents)?;
            log_lines.push(detail);
        }
    }

    // Update lock file
    let lock_path = config::lock_file_path(global);
    let mut lock = LockFile::load(&lock_path).unwrap_or_default();
    lock.version = 1;
    installer::record_install(
        &mut lock,
        &results,
        source.as_deref().unwrap_or("."),
        method,
    );

    // Also record hooks in the lock file
    for harness in &harnesses {
        for h in &selected_hooks {
            let harness_id = harness.id().to_string();
            if let Some(existing) = lock.entries.get_mut(&h.name) {
                if !existing.harnesses.contains(&harness_id) {
                    existing.harnesses.push(harness_id);
                }
            } else {
                lock.add(config::LockEntry {
                    name: h.name.clone(),
                    kind: config::ItemKind::Hook,
                    source: source.as_deref().unwrap_or(".").into(),
                    harnesses: vec![harness_id],
                    method,
                    installed_at: config::now_iso(),
                });
            }
        }
    }

    lock.save(&lock_path)?;

    // Reconcile: update existing agents with newly installed skills/hooks
    reconcile_agents(global, &source_dir, &harnesses)?;

    let scope = if global { "global" } else { "project" };
    let harness_names: Vec<&str> = harnesses.iter().map(|h| h.name()).collect();

    let mut updated_names: Vec<String> = Vec::new();
    for a in &selected_agents {
        if previously_installed.contains(&a.name) {
            updated_names.push(a.name.clone());
        }
    }
    for s in &selected_skills {
        if previously_installed.contains(&s.name) {
            updated_names.push(s.name.clone());
        }
    }
    for h in &selected_hooks {
        if previously_installed.contains(&h.name) {
            updated_names.push(h.name.clone());
        }
    }

    let summary = tui::SummaryData {
        agents: selected_agents.iter().map(|a| a.name.clone()).collect(),
        skills: selected_skills.iter().map(|s| s.name.clone()).collect(),
        hooks: selected_hooks
            .iter()
            .map(|h| (h.name.clone(), h.event.clone()))
            .collect(),
        updated: updated_names,
        harnesses: harness_names.iter().map(|h| h.to_string()).collect(),
        notes: if skipped_harnesses.is_empty() {
            if global {
                harnesses
                    .iter()
                    .flat_map(|h| {
                        h.summary_paths(true).into_iter().map(move |path| {
                            format!("{} path: {}", h.name(), config::display_path(&path))
                        })
                    })
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            let mut notes = vec![format!(
                "Skipped project-only harnesses: {}. Rerun from the target project directory to install those.",
                skipped_harnesses.join(", ")
            )];
            if global {
                notes.extend(harnesses.iter().flat_map(|h| {
                    h.summary_paths(true).into_iter().map(move |path| {
                        format!("{} path: {}", h.name(), config::display_path(&path))
                    })
                }));
            }
            notes
        },
        method: method.to_string(),
        scope: scope.to_string(),
    };

    // Show summary — TUI if interactive, text if non-interactive
    if !yes && !all && harness_filter.is_none() {
        let action = tui::run_summary_screen(&summary)?;
        if action == tui::SummaryAction::InstallMore {
            // Recursive call to restart
            return run(
                Some(resolved_source.source.clone()),
                global,
                harness_filter,
                skill_filter,
                copy,
                yes,
                all,
            );
        }
    } else {
        let counts: Vec<String> = [
            (!summary.agents.is_empty()).then(|| format!("{} agents", summary.agents.len())),
            (!summary.skills.is_empty()).then(|| format!("{} skills", summary.skills.len())),
            (!summary.hooks.is_empty()).then(|| format!("{} hooks", summary.hooks.len())),
        ]
        .into_iter()
        .flatten()
        .collect();
        eprintln!(
            "\n  Installed {} · {} · {} scope\n  → {}",
            counts.join(" · "),
            method,
            scope,
            harness_names.join(", ")
        );
    }

    // Run CLI binary update if requested
    if update_cli {
        eprintln!("\nUpdating vstack binary...\n");
        let _ = crate::commands::update::run();
        eprintln!("\nRestart vstack to use the new version.");
    }

    Ok(())
}

fn resolve_source(source: Option<&str>) -> Result<PathBuf> {
    match source {
        Some(path) if Path::new(path).exists() => Ok(std::fs::canonicalize(path)?),
        Some(source) if looks_like_remote(source) => clone_or_update(source),
        Some(source) => {
            anyhow::bail!(
                "Source not found: {source}\n\
                 Use a local path or GitHub shorthand (owner/repo)"
            );
        }
        None => {
            // Walk up from CWD to find a local vstack repo first
            let mut dir = std::env::current_dir()?;
            loop {
                if is_vstack_root(&dir) {
                    return Ok(dir);
                }
                if !dir.pop() {
                    break;
                }
            }
            // Fall back to default remote repo
            clone_or_update(crate::REPO)
        }
    }
}

fn is_vstack_root(dir: &Path) -> bool {
    // Never match hidden directories (.opencode/, .claude/, .agents/, etc.)
    if dir
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.starts_with('.'))
    {
        return false;
    }
    // vstack.toml is definitive
    if dir.join("vstack.toml").exists() {
        return true;
    }
    // Require at least 2 of 3 standard source dirs
    let count = [
        dir.join("agents").is_dir(),
        dir.join("skills").is_dir(),
        dir.join("hooks").is_dir(),
    ]
    .iter()
    .filter(|&&b| b)
    .count();
    count >= 2
}

fn looks_like_remote(source: &str) -> bool {
    // owner/repo, https://github.com/..., git@github.com:...
    source.contains('/') && !source.starts_with('.') && !source.starts_with('/')
        || source.starts_with("https://")
        || source.starts_with("git@")
}

/// Clone or update a remote repo into ~/.vstack/cache/<owner>/<repo>
fn clone_or_update(source: &str) -> Result<PathBuf> {
    let cache_dir = crate::config::global_base_dir()
        .join(".vstack")
        .join("cache");
    std::fs::create_dir_all(&cache_dir)?;

    // Normalize source to a git URL and a cache key
    let (git_url, cache_key) = if source.starts_with("https://") || source.starts_with("git@") {
        // Full URL — extract owner/repo for cache key
        let key = source
            .trim_end_matches('/')
            .trim_end_matches(".git")
            .rsplit('/')
            .take(2)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("_");
        (source.to_string(), key)
    } else {
        // owner/repo shorthand
        let url = format!("https://github.com/{}.git", source);
        let key = source.replace('/', "_");
        (url, key)
    };

    let repo_dir = cache_dir.join(&cache_key);

    if repo_dir.join(".git").exists() {
        // Update existing clone (handles force-pushed histories)
        eprintln!("Updating cached repo...");
        let fetch = std::process::Command::new("git")
            .args(["fetch", "origin", "--quiet"])
            .current_dir(&repo_dir)
            .status();
        if fetch.is_ok_and(|s| s.success()) {
            let _ = std::process::Command::new("git")
                .args(["reset", "--hard", "origin/HEAD"])
                .current_dir(&repo_dir)
                .stderr(std::process::Stdio::null())
                .status();
        }
    } else {
        // Fresh shallow clone
        eprintln!("Cloning {}...", git_url);
        let status = std::process::Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                &git_url,
                repo_dir.to_str().unwrap(),
            ])
            .status()
            .context("failed to run git clone — is git installed?")?;
        if !status.success() {
            anyhow::bail!(
                "git clone failed. For private repos, make sure you have access:\n\
                 \n\
                 SSH:   git clone git@github.com:{source}.git\n\
                 HTTPS: gh auth login\n\
                 Token: export GH_TOKEN=<your-token>"
            );
        }
    }

    if !is_vstack_root(&repo_dir) {
        anyhow::bail!(
            "Cloned repo doesn't look like a vstack repo (no agents/, skills/, or hooks/ found)"
        );
    }

    Ok(repo_dir)
}

fn reconcile_agents(
    global: bool,
    source_dir: &std::path::Path,
    harnesses: &[Harness],
) -> anyhow::Result<()> {
    let lock_path = config::lock_file_path(global);
    let lock = config::LockFile::load(&lock_path)?;
    let mapping = crate::mapping::MappingConfig::load(source_dir);

    // Collect all installed skill names
    let installed_skills: Vec<String> = lock
        .entries
        .iter()
        .filter(|(_, e)| e.kind == config::ItemKind::Skill)
        .map(|(name, _)| name.clone())
        .collect();

    // Collect all installed agent entries
    let agent_entries: Vec<_> = lock
        .entries
        .iter()
        .filter(|(_, e)| e.kind == config::ItemKind::Agent)
        .collect();

    if agent_entries.is_empty() || installed_skills.is_empty() {
        return Ok(());
    }

    // Discover source agents and skills for descriptions
    let agents_dir = source_dir.join("agents");
    let skills_dir = source_dir.join("skills");
    let hooks_dir = source_dir.join("hooks");

    let source_agents = crate::agent::discover_agents(&agents_dir).unwrap_or_default();
    let source_skills = crate::skill::discover_skills(&skills_dir).unwrap_or_default();
    let source_hooks = crate::hook::discover_hooks(&hooks_dir).unwrap_or_default();

    for (name, entry) in &agent_entries {
        let Some(agent) = source_agents.iter().find(|a| &a.name == *name) else {
            continue;
        };

        let matched_skill_names =
            mapping.skills_for_agent(&agent.name, &agent.role, &installed_skills);
        let skill_pairs: Vec<(String, String)> = matched_skill_names
            .iter()
            .filter_map(|sname| {
                source_skills
                    .iter()
                    .find(|s| &s.name == sname)
                    .map(|s| (s.name.clone(), s.description.clone()))
            })
            .collect();

        let matched_hooks: Vec<crate::hook::Hook> = mapping
            .hooks_for_agent(&agent.role, &source_hooks)
            .into_iter()
            .cloned()
            .collect();

        // Regenerate for each harness this agent is installed to
        for harness_id in &entry.harnesses {
            if let Some(harness) = Harness::from_id(harness_id) {
                // Only reconcile harnesses that were part of this install
                if harnesses.contains(&harness) {
                    let _ = harness.generate_agent(agent, global, &skill_pairs, &matched_hooks);
                }
            }
        }
    }

    Ok(())
}
