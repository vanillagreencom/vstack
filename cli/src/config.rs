use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Lock file entry for tracking installed items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockEntry {
    pub name: String,
    pub kind: ItemKind,
    pub source: String,
    pub harnesses: Vec<String>,
    pub method: InstallMethod,
    pub installed_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemKind {
    Skill,
    Agent,
    Hook,
}

impl std::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemKind::Skill => write!(f, "skill"),
            ItemKind::Agent => write!(f, "agent"),
            ItemKind::Hook => write!(f, "hook"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstallMethod {
    Symlink,
    Copy,
}

impl std::fmt::Display for InstallMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallMethod::Symlink => write!(f, "symlink"),
            InstallMethod::Copy => write!(f, "copy"),
        }
    }
}

/// Lock file tracking all installations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LockFile {
    pub version: u32,
    pub entries: BTreeMap<String, LockEntry>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceRegistry {
    pub current: Option<String>,
    pub entries: Vec<String>,
}

impl LockFile {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                version: 1,
                ..Default::default()
            });
        }
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("reading lock file {}", path.display()))?;
        serde_json::from_str(&content).context("parsing lock file")
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn add(&mut self, entry: LockEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }

    pub fn remove(&mut self, name: &str) -> Option<LockEntry> {
        self.entries.remove(name)
    }
}

impl SourceRegistry {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("reading source registry {}", path.display()))?;
        serde_json::from_str(&content).context("parsing source registry")
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn remember(&mut self, source: &str) {
        if !self.entries.iter().any(|entry| entry == source) {
            self.entries.push(source.to_string());
        }
        self.current = Some(source.to_string());
    }

    pub fn forget(&mut self, source: &str) {
        self.entries.retain(|e| e != source);
        if self.current.as_deref() == Some(source) {
            self.current = None;
        }
    }
}

/// Resolve the lock file path based on scope
pub fn lock_file_path(global: bool) -> PathBuf {
    if global {
        global_state_dir().join(".vstack-lock.json")
    } else {
        project_root().join(".vstack-lock.json")
    }
}

pub fn user_home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"))
}

pub fn user_config_dir() -> PathBuf {
    dirs::config_dir().unwrap_or_else(|| user_home_dir().join(".config"))
}

pub fn global_state_dir() -> PathBuf {
    user_config_dir().join("vstack")
}

pub fn source_registry_path() -> PathBuf {
    global_state_dir().join("sources.json")
}

pub fn display_path(path: &Path) -> String {
    let home = user_home_dir();
    if let Ok(rel) = path.strip_prefix(&home) {
        if rel.as_os_str().is_empty() {
            "~".into()
        } else {
            format!("~/{}", rel.display())
        }
    } else {
        path.display().to_string()
    }
}

/// Base directory for legacy home-scoped global installations.
pub fn global_base_dir() -> PathBuf {
    user_home_dir()
}

pub fn claude_global_dir() -> PathBuf {
    user_home_dir().join(".claude")
}

pub fn cursor_global_dir() -> PathBuf {
    user_home_dir().join(".cursor")
}

pub fn opencode_global_dir() -> PathBuf {
    if let Some(config_path) = std::env::var_os("OPENCODE_CONFIG").map(PathBuf::from)
        && let Some(parent) = config_path.parent()
    {
        return parent.to_path_buf();
    }
    std::env::var_os("OPENCODE_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| user_config_dir().join("opencode"))
}

pub fn opencode_global_config_path() -> PathBuf {
    std::env::var_os("OPENCODE_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|| opencode_global_dir().join("opencode.json"))
}

pub fn opencode_project_config_path() -> PathBuf {
    let root = project_root();
    let json = root.join("opencode.json");
    if json.exists() {
        return json;
    }
    let jsonc = root.join("opencode.jsonc");
    if jsonc.exists() {
        return jsonc;
    }
    json
}

pub fn codex_home_dir() -> PathBuf {
    std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| user_home_dir().join(".codex"))
}

/// Find the project root by walking up from CWD.
/// Looks for `.vstack-lock.json` or harness config dirs.
pub fn project_root() -> PathBuf {
    static ROOT: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    ROOT.get_or_init(find_project_root).clone()
}

fn find_project_root() -> PathBuf {
    let Ok(mut dir) = std::env::current_dir() else {
        return PathBuf::from(".");
    };
    loop {
        if dir.join(".vstack-lock.json").exists()
            || dir.join(".claude").is_dir()
            || dir.join(".cursor").is_dir()
            || dir.join(".codex").is_dir()
            || dir.join(".opencode").is_dir()
            || dir.join(".agents").is_dir()
        {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Get current timestamp as ISO 8601 string (UTC)
pub fn now_iso() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Manual ISO 8601 without chrono: YYYY-MM-DDTHH:MM:SSZ
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;
    // Days since epoch to date (simplified Gregorian)
    let (year, month, day) = epoch_days_to_date(days);
    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

fn epoch_days_to_date(days: u64) -> (u64, u64, u64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
