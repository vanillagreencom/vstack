use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// A hook parsed from a shell script with YAML-in-comments frontmatter.
#[derive(Debug, Clone)]
pub struct Hook {
    pub name: String,
    pub event: String,
    pub matcher: Option<String>,
    pub description: String,
    pub safety: Option<String>,
    pub timeout: Option<u32>,
    /// Full script content
    pub script: String,
    /// Source file path
    pub source_path: PathBuf,
}

impl Hook {
    /// Parse a hook from a shell script file.
    /// Expects YAML-in-comments frontmatter between `# ---` delimiters.
    pub fn from_file(path: &Path) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;

        let meta = parse_hook_frontmatter(&content)
            .with_context(|| format!("parsing hook frontmatter in {}", path.display()))?;

        Ok(Hook {
            name: meta.name,
            event: meta.event,
            matcher: meta.matcher,
            description: meta.description,
            safety: meta.safety,
            timeout: meta.timeout,
            script: content.clone(),
            source_path: path.to_path_buf(),
        })
    }

    /// Generate the safety advisory prose for harnesses without native hook support.
    /// Used by Codex (developer_instructions) and Cursor (rule content).
    pub fn safety_prose(&self) -> String {
        let mut prose = String::new();
        prose.push_str(&format!("**Safety: {}**\n", self.description));
        if let Some(ref safety) = self.safety {
            prose.push_str(&format!("{}\n", safety));
        }
        let action = match self.event.as_str() {
            "PreToolUse" => "Before executing",
            "PostToolUse" => "After executing",
            "PermissionRequest" => "When requesting permission for",
            "PostCompact" => "After context compaction",
            "TaskCompleted" => "Before marking a task complete",
            _ => "When handling",
        };
        let target = self.matcher.as_deref().unwrap_or("any tool");
        prose.push_str(&format!(
            "{action} {target} operations, the agent should verify this constraint is met.\n"
        ));
        prose
    }
}

struct HookMeta {
    name: String,
    event: String,
    matcher: Option<String>,
    description: String,
    safety: Option<String>,
    timeout: Option<u32>,
}

/// Parse YAML-in-comments frontmatter from a shell script.
/// Format:
/// ```
/// # ---
/// # name: hook-name
/// # event: PreToolUse
/// # matcher: Bash
/// # description: What this hook does
/// # ---
/// ```
fn parse_hook_frontmatter(content: &str) -> Result<HookMeta> {
    let mut in_frontmatter = false;
    let mut yaml_lines = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "# ---" {
            if in_frontmatter {
                break;
            } else {
                in_frontmatter = true;
                continue;
            }
        }
        if in_frontmatter {
            // Strip leading "# " from YAML lines
            let yaml_line = if let Some(stripped) = trimmed.strip_prefix("# ") {
                stripped
            } else if let Some(stripped) = trimmed.strip_prefix('#') {
                stripped
            } else {
                anyhow::bail!("unexpected line in hook frontmatter: {}", trimmed);
            };
            yaml_lines.push(yaml_line.to_string());
        }
    }

    if yaml_lines.is_empty() {
        anyhow::bail!("no frontmatter found");
    }

    let yaml_str = yaml_lines.join("\n");
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str).context("parsing hook YAML")?;

    let map = yaml
        .as_mapping()
        .context("hook frontmatter must be a YAML mapping")?;

    let name = map
        .get("name")
        .and_then(|v| v.as_str())
        .context("hook missing 'name' field")?
        .to_string();

    let event = map
        .get("event")
        .and_then(|v| v.as_str())
        .context("hook missing 'event' field")?
        .to_string();

    let matcher = map
        .get("matcher")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    let description = map
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let safety = map
        .get("safety")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let timeout = map
        .get("timeout")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);

    Ok(HookMeta {
        name,
        event,
        matcher,
        description,
        safety,
        timeout,
    })
}

/// Discover all hook scripts in a directory.
pub fn discover_hooks(dir: &Path) -> Result<Vec<Hook>> {
    let mut hooks = Vec::new();
    if !dir.exists() {
        return Ok(hooks);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "sh") {
            match Hook::from_file(&path) {
                Ok(hook) => hooks.push(hook),
                Err(e) => eprintln!("Warning: skipping hook {}: {e}", path.display()),
            }
        }
    }
    hooks.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(hooks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hook_script() {
        let content = r#"#!/usr/bin/env bash
# ---
# name: test-hook
# event: PreToolUse
# matcher: Bash
# description: A test hook
# safety: Prevents bad things
# timeout: 30
# ---

set -euo pipefail
echo "hello"
"#;
        let meta = parse_hook_frontmatter(content).unwrap();
        assert_eq!(meta.name, "test-hook");
        assert_eq!(meta.event, "PreToolUse");
        assert_eq!(meta.matcher.as_deref(), Some("Bash"));
        assert_eq!(meta.description, "A test hook");
        assert_eq!(meta.safety.as_deref(), Some("Prevents bad things"));
        assert_eq!(meta.timeout, Some(30));
    }

    #[test]
    fn parse_hook_no_matcher() {
        let content = r#"#!/usr/bin/env bash
# ---
# name: post-compact
# event: PostCompact
# matcher:
# description: Warn after compaction
# ---

echo "warning"
"#;
        let meta = parse_hook_frontmatter(content).unwrap();
        assert_eq!(meta.name, "post-compact");
        assert!(meta.matcher.is_none());
    }
}
