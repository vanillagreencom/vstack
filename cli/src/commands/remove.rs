use crate::config::{self, LockFile};
use crate::harness::Harness;
use crate::installer;
use anyhow::Result;

pub fn run(names: &[String], global: bool) -> Result<()> {
    if names.is_empty() {
        eprintln!("Usage: vstack remove <name> [<name>...]");
        return Ok(());
    }

    let lock_path = config::lock_file_path(global);
    let mut lock = LockFile::load(&lock_path).unwrap_or_default();

    for name in names {
        // Look up harnesses from lock file, or try all
        let harnesses: Vec<Harness> = if let Some(entry) = lock.entries.get(name.as_str()) {
            entry
                .harnesses
                .iter()
                .filter_map(|h| Harness::from_id(h))
                .collect()
        } else {
            Harness::ALL.to_vec()
        };

        let removed = installer::remove_item(name, &harnesses, global)?;

        if removed.is_empty() {
            eprintln!("  {name}: not found");
        } else {
            for path in &removed {
                eprintln!("  removed {}", path.display());
            }
            lock.remove(name);
        }
    }

    lock.save(&lock_path)?;
    Ok(())
}
