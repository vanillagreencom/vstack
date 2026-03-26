use crate::config::{self, LockEntry, LockFile};
use crate::harness::Harness;
use anyhow::Result;

pub fn run() -> Result<()> {
    // Check CLI version
    let local_hash = env!("VSTACK_GIT_HASH");
    eprintln!("vstack {} ({})", env!("CARGO_PKG_VERSION"), local_hash);

    if let Some(remote) = crate::commands::update::get_remote_hash() {
        let remote_short = &remote[..7.min(remote.len())];
        let local_short = &local_hash[..7.min(local_hash.len())];
        if !remote.starts_with(local_hash) && !local_hash.starts_with(remote_short) {
            eprintln!(
                "  CLI update available: {local_short} → {remote_short}  (run: vstack update)"
            );
        } else {
            eprintln!("  CLI is up to date.");
        }
    }

    // Check installed items
    for global in [false, true] {
        let lock_path = config::lock_file_path(global);
        let lock = LockFile::load(&lock_path)?;

        let scope = if global { "global" } else { "project" };

        if lock.entries.is_empty() {
            continue;
        }

        eprintln!("\n{scope} scope: {} item(s)", lock.entries.len());

        let mut outdated = 0;
        for entry in lock.entries.values() {
            let status = check_staleness(entry, global);
            if status == "outdated" {
                outdated += 1;
            }
            let icon = match status {
                "ok" => "✓",
                "outdated" => "!",
                _ => "?",
            };
            eprintln!(
                "  {icon} {} ({}){}",
                entry.name,
                entry.kind,
                if status == "outdated" {
                    "  ← outdated"
                } else {
                    ""
                }
            );
        }

        if outdated > 0 {
            eprintln!("\n  {outdated} outdated — run `vstack add` to update");
        }
    }

    Ok(())
}

fn skill_install_paths(entry: &LockEntry, global: bool) -> Vec<std::path::PathBuf> {
    if global {
        let mut paths = Vec::new();
        if entry
            .harnesses
            .iter()
            .filter_map(|h| Harness::from_id(h))
            .any(|h| !matches!(h, Harness::Codex))
        {
            paths.push(config::global_state_dir().join("skills").join(&entry.name));
        }
        if entry
            .harnesses
            .iter()
            .filter_map(|h| Harness::from_id(h))
            .any(|h| matches!(h, Harness::Codex))
        {
            paths.push(config::codex_home_dir().join("skills").join(&entry.name));
        }
        paths
    } else {
        vec![
            config::project_root()
                .join(".agents")
                .join("skills")
                .join(&entry.name),
        ]
    }
}

fn hook_install_paths(entry: &LockEntry, global: bool) -> Vec<std::path::PathBuf> {
    entry
        .harnesses
        .iter()
        .filter_map(|h| Harness::from_id(h))
        .filter(|h| matches!(h, Harness::ClaudeCode))
        .map(|_| {
            if global {
                config::claude_global_dir()
                    .join("hooks")
                    .join(format!("{}.sh", entry.name))
            } else {
                config::project_root()
                    .join(".claude")
                    .join("hooks")
                    .join(format!("{}.sh", entry.name))
            }
        })
        .collect()
}

fn check_staleness(entry: &LockEntry, global: bool) -> &'static str {
    let root = config::project_root();
    match entry.kind {
        config::ItemKind::Skill => {
            let installed_paths = skill_install_paths(entry, global);
            if installed_paths.is_empty() {
                return "missing";
            }
            if installed_paths.iter().any(|path| !path.exists()) {
                return "missing";
            }
            // Walk up to find source — check common locations
            for source_base in std::slice::from_ref(&root) {
                let source = source_base
                    .join("skills")
                    .join(&entry.name)
                    .join("SKILL.md");
                if source.exists() {
                    let Ok(src) = std::fs::read_to_string(&source) else {
                        continue;
                    };
                    for installed_root in &installed_paths {
                        let installed = installed_root.join("SKILL.md");
                        if let Ok(inst) = std::fs::read_to_string(&installed)
                            && inst != src
                        {
                            return "outdated";
                        }
                    }
                }
            }
            "ok"
        }
        config::ItemKind::Hook => {
            let installed_paths = hook_install_paths(entry, global);
            if installed_paths.is_empty() {
                return "ok"; // hooks may not exist for all harnesses
            }
            if installed_paths.iter().any(|path| !path.exists()) {
                return "missing";
            }
            "ok"
        }
        config::ItemKind::Agent => "ok", // agents are regenerated, not compared
    }
}
