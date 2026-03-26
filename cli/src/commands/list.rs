use crate::config::{self, LockFile};
use crate::harness::Harness;
use anyhow::Result;

pub fn run(global: bool, agent_filter: Option<&str>) -> Result<()> {
    let lock_path = config::lock_file_path(global);
    let lock = LockFile::load(&lock_path)?;

    let scope = if global { "global" } else { "project" };

    if lock.entries.is_empty() {
        eprintln!("No items installed ({scope} scope).");
        return Ok(());
    }

    eprintln!("Installed items ({scope} scope):\n");

    // Group by kind
    let mut agents = Vec::new();
    let mut skills = Vec::new();
    let mut hooks = Vec::new();

    for entry in lock.entries.values() {
        // Apply harness filter
        if let Some(filter) = agent_filter
            && let Some(harness) = Harness::from_id(filter)
            && !entry
                .harnesses
                .iter()
                .any(|installed| installed == harness.id())
        {
            continue;
        }

        match entry.kind {
            crate::config::ItemKind::Agent => agents.push(entry),
            crate::config::ItemKind::Skill => skills.push(entry),
            crate::config::ItemKind::Hook => hooks.push(entry),
        }
    }

    let mut printed = false;

    for (label, items) in [("Agents", &agents), ("Skills", &skills), ("Hooks", &hooks)] {
        if items.is_empty() {
            continue;
        }
        if printed {
            eprintln!();
        }
        eprintln!("  {label}:");
        for entry in items {
            let harnesses = entry.harnesses.join(", ");
            eprintln!("    {} ({}) [{}]", entry.name, entry.method, harnesses);
        }
        printed = true;
    }

    eprintln!(
        "\n  Total: {} agent(s), {} skill(s), {} hook(s)",
        agents.len(),
        skills.len(),
        hooks.len()
    );
    Ok(())
}
