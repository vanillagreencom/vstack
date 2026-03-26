use anyhow::{Context, Result};
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const REPO: &str = "vanillagreencom/vstack";

pub fn run() -> Result<()> {
    let local_hash = env!("VSTACK_GIT_HASH");

    eprintln!("vstack {} ({})", env!("CARGO_PKG_VERSION"), local_hash);
    eprintln!("Checking for updates...");

    // Check latest remote commit
    match get_remote_hash() {
        Some(remote_hash) => {
            if remote_hash.starts_with(local_hash) || local_hash.starts_with(&remote_hash) {
                eprintln!("Already up to date.");
                return Ok(());
            }
            eprintln!(
                "Update available: {} → {}",
                &local_hash[..7.min(local_hash.len())],
                &remote_hash[..7.min(remote_hash.len())]
            );
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

/// Check latest commit hash on remote main branch
pub fn get_remote_hash() -> Option<String> {
    get_remote_hash_inner(None)
}

pub fn get_remote_hash_with_timeout(timeout: Duration) -> Option<String> {
    get_remote_hash_inner(Some(timeout))
}

fn get_remote_hash_inner(timeout: Option<Duration>) -> Option<String> {
    let mut child = Command::new("git")
        .args([
            "ls-remote",
            &format!("https://github.com/{REPO}.git"),
            "HEAD",
        ])
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
    let hash = stdout.split_whitespace().next()?;
    Some(hash.to_string())
}

/// Check if an update is available (called from other commands)
pub fn check_update_hint() {
    let local = env!("VSTACK_GIT_HASH");

    // Quick non-blocking check
    if let Some(remote) = get_remote_hash() {
        let remote_short = &remote[..7.min(remote.len())];
        let local_short = &local[..7.min(local.len())];
        if !remote.starts_with(local) && !local.starts_with(remote_short) {
            eprintln!(
                "\n  Update available: {} → {}  (run: vstack update)",
                local_short, remote_short
            );
        }
    }
}
