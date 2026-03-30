#![allow(dead_code)]

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Canonical agent definition — harness-agnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub description: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default)]
    pub role: AgentRole,
    #[serde(default)]
    pub color: Option<String>,
    /// Body markdown (everything after frontmatter)
    #[serde(skip)]
    pub body: String,
}

fn default_model() -> String {
    "sonnet".into()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AgentRole {
    Reviewer,
    #[default]
    Engineer,
    Manager,
}

impl AgentRole {
    /// Whether this role writes code
    pub fn writes_code(&self) -> bool {
        matches!(self, AgentRole::Engineer)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AgentRole::Reviewer => "reviewer",
            AgentRole::Engineer => "engineer",
            AgentRole::Manager => "manager",
        }
    }
}

impl std::fmt::Display for AgentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Agent {
    /// Parse a canonical agent file (YAML frontmatter + markdown body)
    pub fn from_file(path: &Path) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        Self::parse(&content)
    }

    /// Parse from string content
    pub fn parse(content: &str) -> Result<Self> {
        let (frontmatter, body) = crate::frontmatter::split_yaml_frontmatter(content)?;
        let mut agent: Agent =
            serde_yaml::from_str(&frontmatter).context("parsing agent frontmatter")?;
        agent.body = body;
        Ok(agent)
    }

    /// Map model name to provider-specific model ID
    pub fn model_id(&self, provider: &str) -> String {
        let base = self.model.to_lowercase();
        match provider {
            "anthropic" => match base.as_str() {
                "opus" => "anthropic/claude-opus-4-20250514".into(),
                "sonnet" => "anthropic/claude-sonnet-4-20250514".into(),
                "haiku" => "anthropic/claude-haiku-4-5-20251001".into(),
                other => other.into(),
            },
            "openai" => match base.as_str() {
                "opus" => "openai/gpt-5.4".into(),
                "sonnet" => "openai/gpt-5.4".into(),
                "haiku" => "openai/gpt-5.4".into(),
                other => format!("openai/{other}"),
            },
            "claude-code" => match base.as_str() {
                "opus" => "opus[1m]".into(),
                "sonnet" => "sonnet".into(),
                "haiku" => "haiku".into(),
                other => other.into(),
            },
            _ => base,
        }
    }
}

/// Discover all agent files in a directory
pub fn discover_agents(dir: &Path) -> Result<Vec<Agent>> {
    let mut agents = Vec::new();
    if !dir.exists() {
        return Ok(agents);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "md") {
            match Agent::from_file(&path) {
                Ok(agent) => agents.push(agent),
                Err(e) => eprintln!("Warning: skipping {}: {e}", path.display()),
            }
        }
    }
    agents.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(agents)
}

pub fn skill_match_prefix(agent_name: &str) -> &str {
    agent_name.strip_prefix("reviewer-").unwrap_or(agent_name)
}

pub fn prefixed_skill_matches(agent_name: &str, available: &[String]) -> Vec<String> {
    let mut matched = Vec::new();
    let name = agent_name.to_lowercase();
    let prefix = skill_match_prefix(&name);

    for skill in available {
        if skill.starts_with(&format!("{prefix}-")) || skill == prefix {
            matched.push(skill.clone());
        }
    }

    matched
}

fn default_role_skills(agent_role: &AgentRole) -> &'static [&'static str] {
    match agent_role {
        AgentRole::Reviewer => &["issue-lifecycle"],
        AgentRole::Engineer => &["issue-lifecycle", "github", "worktree"],
        AgentRole::Manager => &[
            "project-management",
            "linear",
            "issue-lifecycle",
            "github",
            "worktree",
        ],
    }
}

/// Match skills to an agent by name prefix and role
pub fn match_skills(agent_name: &str, agent_role: &AgentRole, available: &[String]) -> Vec<String> {
    let mut matched = prefixed_skill_matches(agent_name, available);

    for skill_name in default_role_skills(agent_role) {
        if available.iter().any(|skill| skill == skill_name)
            && !matched.iter().any(|skill| skill == skill_name)
        {
            matched.push((*skill_name).to_string());
        }
    }

    matched.sort();
    matched.dedup();
    matched
}

/// Match hooks to an agent based on role
pub fn match_hooks<'a>(
    agent_role: &AgentRole,
    hooks: &'a [crate::hook::Hook],
) -> Vec<&'a crate::hook::Hook> {
    hooks
        .iter()
        .filter(|h| {
            match agent_role {
                AgentRole::Engineer => true,
                AgentRole::Reviewer | AgentRole::Manager => {
                    // Get Bash safety hooks and lifecycle hooks, not edit/write hooks
                    h.event == "PostCompact"
                        || h.event == "TaskCompleted"
                        || (h.event == "PreToolUse" && h.matcher.as_deref() == Some("Bash"))
                        || (h.event == "PostToolUse" && h.matcher.as_deref() == Some("Bash"))
                }
            }
        })
        .collect()
}

/// Per-agent customization from project-level config
#[derive(Debug, Clone, Default)]
pub struct AgentExtras {
    pub guidance: Option<String>,
    pub instructions: Option<String>,
}

/// Generate an "Execute on Launch" markdown section
pub fn guidance_section(text: Option<&str>) -> String {
    match text {
        Some(t) if !t.is_empty() => format!("## Execute on Launch\n\n{}\n\n", t.trim()),
        _ => String::new(),
    }
}

/// Generate an "Additional Instructions" markdown section
pub fn instructions_section(text: Option<&str>) -> String {
    match text {
        Some(t) if !t.is_empty() => format!("## Additional Instructions\n\n{}\n", t.trim()),
        _ => String::new(),
    }
}

/// Append a section to the end of a markdown body
pub fn append_section(body: &str, section: &str) -> String {
    if section.is_empty() {
        return body.to_string();
    }
    let trimmed = body.trim_end();
    format!("{}\n\n{}\n", trimmed, section.trim_end())
}

/// Extract user-edited "When to Use" and "Additional Instructions" sections
/// from an existing generated agent file so they can be preserved across regeneration.
pub fn extract_user_sections(content: &str) -> AgentExtras {
    AgentExtras {
        guidance: extract_section(content, "## Execute on Launch")
            .or_else(|| extract_section(content, "## When to Use")),
        instructions: extract_section(content, "## Additional Instructions"),
    }
}

/// Extract a markdown section's body text between its heading and the next `## ` heading.
fn extract_section(content: &str, header: &str) -> Option<String> {
    let start = content.find(header)?;
    let after_header = &content[start + header.len()..];
    // Find the body text (skip leading whitespace)
    let trimmed = after_header.trim_start();
    if trimmed.is_empty() {
        return None;
    }
    // End at next ## heading or end of content
    let end = trimmed.find("\n## ").unwrap_or(trimmed.len());
    let text = trimmed[..end].trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

/// Extract the developer_instructions body from a Codex TOML agent file.
pub fn extract_body_from_codex_toml(content: &str) -> Option<String> {
    let marker = "developer_instructions = '''\n";
    let start = content.find(marker)?;
    let after = &content[start + marker.len()..];
    let end = after.find("'''")?;
    Some(after[..end].to_string())
}

/// Generate a "Load These Skills" markdown section
pub fn load_skills_section(skills: &[(String, String)]) -> String {
    if skills.is_empty() {
        return String::new();
    }
    let mut section = String::from(
        "## Load These Skills\n\nLoad the relevant skill before working on these areas:\n\n",
    );
    for (name, desc) in skills {
        section.push_str(&format!("- **{}** → `{}`\n", desc, name));
    }
    section
}

/// Insert a section after the first heading block in markdown body.
/// Finds the first `## ` line and inserts before it.
/// If no `## ` found, appends to the end.
pub fn insert_after_intro(body: &str, section: &str) -> String {
    if section.is_empty() {
        return body.to_string();
    }
    // Find second heading (first ## after the opening # title)
    if let Some(pos) = body.find("\n## ") {
        let insert_at = pos + 1; // after the newline
        format!(
            "{}\n{}\n{}",
            &body[..insert_at],
            section,
            &body[insert_at..]
        )
    } else {
        // No ## found, append with spacing
        format!("{}\n\n{}\n", body.trim_end(), section)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_agent() {
        let content = r#"---
name: test-agent
description: A test agent
model: opus
role: reviewer
color: red
---

# Test Agent

Does testing things.
"#;
        let agent = Agent::parse(content).unwrap();
        assert_eq!(agent.name, "test-agent");
        assert_eq!(agent.role, AgentRole::Reviewer);
        assert!(agent.body.contains("# Test Agent"));
    }

    #[test]
    fn match_skills_by_prefix() {
        let available = vec![
            "rust-arch".into(),
            "rust-async".into(),
            "python-web".into(),
            "issue-lifecycle".into(),
            "github".into(),
            "worktree".into(),
        ];
        let matched = match_skills("rust", &AgentRole::Engineer, &available);
        assert!(matched.contains(&"rust-arch".to_string()));
        assert!(matched.contains(&"rust-async".to_string()));
        assert!(!matched.contains(&"python-web".to_string()));
        // Engineer gets workflow skills
        assert!(matched.contains(&"issue-lifecycle".to_string()));
        assert!(matched.contains(&"github".to_string()));
        assert!(matched.contains(&"worktree".to_string()));
    }

    #[test]
    fn match_skills_reviewer_prefix_strip() {
        let available = vec![
            "rust-arch".into(),
            "rust-async".into(),
            "issue-lifecycle".into(),
        ];
        let matched = match_skills("reviewer-rust", &AgentRole::Reviewer, &available);
        assert!(matched.contains(&"rust-arch".to_string()));
        assert!(matched.contains(&"rust-async".to_string()));
        assert!(matched.contains(&"issue-lifecycle".to_string()));
    }

    #[test]
    fn match_hooks_engineer_gets_all() {
        let hooks = vec![
            crate::hook::Hook {
                name: "h1".into(),
                event: "PreToolUse".into(),
                matcher: Some("Bash".into()),
                description: "".into(),
                safety: None,
                timeout: None,
                script: "".into(),
                source_path: std::path::PathBuf::new(),
            },
            crate::hook::Hook {
                name: "h2".into(),
                event: "PostToolUse".into(),
                matcher: Some("Edit|Write".into()),
                description: "".into(),
                safety: None,
                timeout: None,
                script: "".into(),
                source_path: std::path::PathBuf::new(),
            },
        ];
        let matched = match_hooks(&AgentRole::Engineer, &hooks);
        assert_eq!(matched.len(), 2);
    }

    #[test]
    fn match_hooks_reviewer_filters() {
        let hooks = vec![
            crate::hook::Hook {
                name: "h1".into(),
                event: "PreToolUse".into(),
                matcher: Some("Bash".into()),
                description: "".into(),
                safety: None,
                timeout: None,
                script: "".into(),
                source_path: std::path::PathBuf::new(),
            },
            crate::hook::Hook {
                name: "h2".into(),
                event: "PostToolUse".into(),
                matcher: Some("Edit|Write".into()),
                description: "".into(),
                safety: None,
                timeout: None,
                script: "".into(),
                source_path: std::path::PathBuf::new(),
            },
            crate::hook::Hook {
                name: "h3".into(),
                event: "PostCompact".into(),
                matcher: None,
                description: "".into(),
                safety: None,
                timeout: None,
                script: "".into(),
                source_path: std::path::PathBuf::new(),
            },
        ];
        let matched = match_hooks(&AgentRole::Reviewer, &hooks);
        // Should get h1 (Bash PreToolUse) and h3 (PostCompact), but not h2 (Edit|Write)
        assert_eq!(matched.len(), 2);
        assert!(matched.iter().any(|h| h.name == "h1"));
        assert!(matched.iter().any(|h| h.name == "h3"));
    }

    #[test]
    fn load_skills_section_empty() {
        assert_eq!(load_skills_section(&[]), String::new());
    }

    #[test]
    fn load_skills_section_format() {
        let skills = vec![
            (
                "rust-arch".into(),
                "Architecture patterns for Rust. More details here.".into(),
            ),
            ("github".into(), "GitHub CLI integration".into()),
        ];
        let section = load_skills_section(&skills);
        assert!(section.contains("## Load These Skills"));
        assert!(
            section
                .contains("**Architecture patterns for Rust. More details here.** → `rust-arch`")
        );
        assert!(section.contains("**GitHub CLI integration** → `github`"));
    }

    #[test]
    fn guidance_section_renders() {
        let section = guidance_section(Some("Read the open issues and start working."));
        assert!(section.contains("## Execute on Launch"));
        assert!(section.contains("Read the open issues and start working."));
    }

    #[test]
    fn guidance_section_empty_on_none() {
        assert_eq!(guidance_section(None), String::new());
        assert_eq!(guidance_section(Some("")), String::new());
    }

    #[test]
    fn instructions_section_renders() {
        let section = instructions_section(Some("Always run clippy."));
        assert!(section.contains("## Additional Instructions"));
        assert!(section.contains("Always run clippy."));
    }

    #[test]
    fn instructions_section_empty_on_none() {
        assert_eq!(instructions_section(None), String::new());
        assert_eq!(instructions_section(Some("")), String::new());
    }

    #[test]
    fn append_section_adds_to_end() {
        let body = "# Title\n\nSome content.\n";
        let section = "## Extra\n\nMore stuff.\n";
        let result = append_section(body, section);
        assert!(result.ends_with("More stuff.\n"));
        assert!(result.contains("Some content."));
    }

    #[test]
    fn append_section_noop_when_empty() {
        let body = "# Title\n\nContent.\n";
        assert_eq!(append_section(body, ""), body.to_string());
    }

    #[test]
    fn extract_user_sections_both() {
        let content = r#"# Agent

Some intro.

## When to Use

Use for backend services.

## Load These Skills

- **Skill** → `skill-name`

## Capabilities

Does stuff.

## Additional Instructions

Always run clippy.
"#;
        let extras = extract_user_sections(content);
        assert_eq!(
            extras.guidance.as_deref(),
            Some("Use for backend services.")
        );
        assert_eq!(
            extras.instructions.as_deref(),
            Some("Always run clippy.")
        );
    }

    #[test]
    fn extract_user_sections_none() {
        let content = "# Agent\n\nJust an intro.\n\n## Capabilities\n\nDoes stuff.\n";
        let extras = extract_user_sections(content);
        assert!(extras.guidance.is_none());
        assert!(extras.instructions.is_none());
    }

    #[test]
    fn extract_body_from_codex() {
        let content = r#"name = "rust"
developer_instructions = '''
# Rust Agent

## Additional Instructions

Use zero-copy APIs.
'''
"#;
        let body = extract_body_from_codex_toml(content).unwrap();
        let extras = extract_user_sections(&body);
        assert_eq!(extras.instructions.as_deref(), Some("Use zero-copy APIs."));
    }
}
