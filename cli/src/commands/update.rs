use anyhow::{Context, Result};
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const REPO: &str = "vanillagreencom/vstack";

pub fn run() -> Result<()> {
    let local_version = env!("CARGO_PKG_VERSION");
    let local_hash = env!("VSTACK_GIT_HASH");

    eprintln!("vstack {} ({})", local_version, local_hash);
    eprintln!("Checking for updates...");

    match get_remote_version() {
        Some(remote_version) => {
            if remote_version == local_version {
                eprintln!("Already up to date.");
                return Ok(());
            }
            eprintln!("Update available: {} → {}", local_version, remote_version);
        }
        None => {
            eprintln!("Could not check remote version, updating anyway...");
        }
    }

    eprintln!("Updating...\n");

    let status = Command::new("cargo")
        .args([
            "install",
            "--git",
            &format!("https://github.com/{REPO}.git"),
            "vstack",
            "--force",
        ])
        .status()
        .context("failed to run cargo install")?;

    if status.success() {
        eprintln!("\nUpdated successfully.");
    } else {
        anyhow::bail!("cargo install failed");
    }

    Ok(())
}

/// Fetch the version string from the remote Cargo.toml (blocking)
pub fn get_remote_version() -> Option<String> {
    get_remote_version_inner(None)
}

/// Fetch the version string from the remote Cargo.toml (with timeout)
pub fn get_remote_version_with_timeout(timeout: Duration) -> Option<String> {
    get_remote_version_inner(Some(timeout))
}

fn get_remote_version_inner(timeout: Option<Duration>) -> Option<String> {
    let url = format!(
        "https://raw.githubusercontent.com/{REPO}/main/cli/Cargo.toml"
    );

    let mut child = Command::new("curl")
        .args(["-sfL", "--max-time", "5", &url])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    let success = if let Some(timeout) = timeout {
        let deadline = Instant::now() + timeout;
        loop {
            match child.try_wait().ok()? {
                Some(status) => break status.success(),
                None if Instant::now() < deadline => thread::sleep(Duration::from_millis(25)),
                None => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return None;
                }
            }
        }
    } else {
        child.wait().ok()?.success()
    };

    if !success {
        return None;
    }

    let mut stdout = String::new();
    child.stdout.take()?.read_to_string(&mut stdout).ok()?;
    parse_cargo_version(&stdout)
}

fn parse_cargo_version(cargo_toml: &str) -> Option<String> {
    for line in cargo_toml.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("version = \"") {
            return rest.strip_suffix('"').map(String::from);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_from_cargo_toml() {
        let toml = r#"[package]
name = "vstack"
version = "0.9.0"
edition = "2024"
"#;
        assert_eq!(parse_cargo_version(toml), Some("0.9.0".into()));
    }

    #[test]
    fn parse_version_missing() {
        assert_eq!(parse_cargo_version("name = \"vstack\""), None);
    }

    #[test]
    fn parse_version_with_prerelease() {
        let toml = "version = \"1.2.3-beta.1\"";
        assert_eq!(parse_cargo_version(toml), Some("1.2.3-beta.1".into()));
    }
}

/// Check if an update is available (called after non-interactive installs)
pub fn check_update_hint() {
    let local = env!("CARGO_PKG_VERSION");

    if let Some(remote) = get_remote_version_with_timeout(Duration::from_millis(1500)) {
        if remote != local {
            eprintln!(
                "\n  Update available: {} → {}  (run: vstack update)",
                local, remote
            );
        }
    }
}
