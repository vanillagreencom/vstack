use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Custom skill entry from project config
#[derive(Debug, Clone, Deserialize)]
pub struct CustomSkill {
    pub name: String,
    pub description: String,
}

/// Project-level agent customization config.
///
/// Loaded from `vstack.toml` at the project root. These sections are
/// independent of the source repo's mapping sections (`[agent-skills]`,
/// `[role-skills]`, `[hook-events]`) and survive updates.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ProjectConfig {
    #[serde(rename = "custom-skills")]
    pub custom_skills: HashMap<String, Vec<CustomSkill>>,
    #[serde(rename = "agent-guidance")]
    pub agent_guidance: HashMap<String, String>,
    #[serde(rename = "agent-instructions")]
    pub agent_instructions: HashMap<String, String>,
}

impl ProjectConfig {
    /// Load project config from a directory's `vstack.toml`.
    /// Returns default (empty) if the file is missing or unparseable.
    pub fn load(project_root: &Path) -> Self {
        let path = project_root.join("vstack.toml");
        if !path.exists() {
            return Self::default();
        }
        let Ok(content) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str(&content).unwrap_or_default()
    }

    /// Get custom skill pairs for an agent
    pub fn custom_skills_for(&self, agent_name: &str) -> Vec<(String, String)> {
        self.custom_skills
            .get(agent_name)
            .map(|skills| {
                skills
                    .iter()
                    .map(|s| (s.name.clone(), s.description.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get guidance text for an agent
    pub fn guidance_for(&self, agent_name: &str) -> Option<&str> {
        self.agent_guidance.get(agent_name).map(|s| s.as_str())
    }

    /// Get additional instructions for an agent
    pub fn instructions_for(&self, agent_name: &str) -> Option<&str> {
        self.agent_instructions.get(agent_name).map(|s| s.as_str())
    }

    /// Merge extracted agent sections into vstack.toml, preserving existing entries.
    /// Only writes new entries — never overwrites user-set values.
    pub fn save_extracted(
        &mut self,
        project_root: &Path,
        agent_name: &str,
        extracted: &crate::agent::AgentExtras,
    ) {
        let needs_guidance =
            extracted.guidance.is_some() && self.guidance_for(agent_name).is_none();
        let needs_instructions =
            extracted.instructions.is_some() && self.instructions_for(agent_name).is_none();

        if !needs_guidance && !needs_instructions {
            return;
        }

        if let Some(ref text) = extracted.guidance {
            if needs_guidance {
                self.agent_guidance
                    .insert(agent_name.to_string(), text.clone());
            }
        }
        if let Some(ref text) = extracted.instructions {
            if needs_instructions {
                self.agent_instructions
                    .insert(agent_name.to_string(), text.clone());
            }
        }

        // Write back to vstack.toml using toml::Value to preserve structure
        let path = project_root.join("vstack.toml");
        let mut doc: toml::Value = if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|c| toml::from_str(&c).ok())
                .unwrap_or(toml::Value::Table(Default::default()))
        } else {
            toml::Value::Table(Default::default())
        };

        let table = doc.as_table_mut().unwrap();

        if needs_guidance {
            if let Some(ref text) = extracted.guidance {
                let section = table
                    .entry("agent-guidance")
                    .or_insert_with(|| toml::Value::Table(Default::default()));
                if let Some(t) = section.as_table_mut() {
                    t.entry(agent_name)
                        .or_insert_with(|| toml::Value::String(text.clone()));
                }
            }
        }

        if needs_instructions {
            if let Some(ref text) = extracted.instructions {
                let section = table
                    .entry("agent-instructions")
                    .or_insert_with(|| toml::Value::Table(Default::default()));
                if let Some(t) = section.as_table_mut() {
                    t.entry(agent_name)
                        .or_insert_with(|| toml::Value::String(text.clone()));
                }
            }
        }

        let _ = std::fs::write(&path, toml::to_string_pretty(&doc).unwrap_or_default());
    }
}

/// Create or update vstack.toml at the project root.
///
/// - If the file doesn't exist, generates a full template with commented placeholders.
/// - If the file exists, appends commented placeholders for any new agents/skills
///   not already mentioned, and updates the installed-skills reference block.
///   Never modifies existing user content.
pub fn ensure_project_config(project_root: &Path, agents: &[String], skills: &[String]) {
    let path = project_root.join("vstack.toml");

    if path.exists() {
        update_project_config(&path, agents, skills);
    } else {
        create_project_config(&path, agents, skills);
    }
}

fn create_project_config(path: &Path, agents: &[String], skills: &[String]) {
    let mut out = String::new();

    out.push_str("# ─────────────────────────────────────────────────────\n");
    out.push_str("# vstack.toml — project-level agent customization\n");
    out.push_str("#\n");
    out.push_str("# Customize agent behavior for this project. These\n");
    out.push_str("# settings are merged into generated agent files on\n");
    out.push_str("# every install and refresh.\n");
    out.push_str("#\n");
    out.push_str("# After editing, run:  vstack refresh\n");
    out.push_str("# ─────────────────────────────────────────────────────\n");
    out.push('\n');

    // ── agent-guidance ──
    out.push_str("\n# ── When to Use ──────────────────────────────────────\n");
    out.push_str("# Adds a \"## When to Use\" section in each agent file,\n");
    out.push_str("# after the agent's built-in description. Tells the\n");
    out.push_str("# AI when to pick this agent for your project.\n");
    out.push_str("#\n");
    out.push_str("[agent-guidance]\n");
    for name in agents {
        out.push_str(&format!("{} = \"\"\n", name));
    }

    // ── agent-instructions ──
    out.push_str("\n\n# ── Additional Instructions ──────────────────────────\n");
    out.push_str("# Adds a \"## Additional Instructions\" section at the\n");
    out.push_str("# bottom of each agent file. Project-specific rules,\n");
    out.push_str("# conventions, or reminders for this agent.\n");
    out.push_str("#\n");
    out.push_str("[agent-instructions]\n");
    for name in agents {
        out.push_str(&format!("{} = \"\"\n", name));
    }

    // ── custom-skills ──
    out.push_str("\n\n# ── Custom Skills ────────────────────────────────────\n");
    out.push_str("# Attach extra skills to agents beyond automatic\n");
    out.push_str("# prefix matching. Each entry is a list of\n");
    out.push_str("# { name, description } objects.\n");
    out.push_str("#\n");
    out.push_str("# [custom-skills]\n");
    if !agents.is_empty() {
        out.push_str(&format!(
            "# {} = [\n#   {{ name = \"my-skill\", description = \"What this skill does\" }},\n# ]\n",
            agents[0]
        ));
    } else {
        out.push_str("# my-agent = [\n#   { name = \"my-skill\", description = \"What this skill does\" },\n# ]\n");
    }

    append_skills_reference(&mut out, skills);
    let _ = std::fs::write(path, out);
}

fn update_project_config(path: &Path, agents: &[String], skills: &[String]) {
    let Ok(existing) = std::fs::read_to_string(path) else {
        return;
    };

    // Find agents not already mentioned as a TOML key (commented or active).
    let new_agents: Vec<&String> = agents
        .iter()
        .filter(|name| agent_mentioned_in(&existing, name))
        .collect();

    // Strip old skills reference block and re-append with current list
    let content = strip_skills_reference(&existing);
    let mut out = content.trim_end().to_string();
    out.push('\n');

    if !new_agents.is_empty() {
        out = insert_keys_into_sections(&out, &new_agents);
    }

    append_skills_reference(&mut out, skills);
    let _ = std::fs::write(path, out);
}

/// Check if an agent name already appears as a TOML key in the file.
fn agent_mentioned_in(content: &str, name: &str) -> bool {
    let patterns = [
        format!("{} =", name),
        format!("{}=", name),
        format!("# {} =", name),
        format!("# {}=", name),
    ];
    !patterns.iter().any(|p| content.contains(p))
}

fn append_skills_reference(out: &mut String, skills: &[String]) {
    if !skills.is_empty() {
        out.push_str("\n\n# ── Installed skills (reference) ─────────────────────\n");
        for name in skills {
            out.push_str(&format!("#   {}\n", name));
        }
    }
}

/// Insert new agent keys into existing [agent-guidance] and [agent-instructions]
/// sections, preserving all other content including comments.
fn insert_keys_into_sections(content: &str, new_agents: &[&String]) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result: Vec<String> = Vec::new();

    let target_sections = ["[agent-guidance]", "[agent-instructions]"];
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        result.push(lines[i].to_string());

        // Check if this line is a target section header
        if target_sections.contains(&trimmed) {
            // Scan forward to find the last key = value line in this section
            i += 1;
            while i < lines.len() {
                let next = lines[i].trim();
                // Stop at next section header, a comment-only divider line, or blank line
                // that's followed by a section header or divider
                let is_key_line = next.contains(" = ") || next.contains("= ");
                let is_comment = next.starts_with('#');
                let is_blank = next.is_empty();

                if next.starts_with('[') && !next.starts_with("# [") {
                    // Hit next section — insert before it
                    break;
                }

                if is_blank || (is_comment && next.starts_with("# ──")) {
                    // Blank line or section divider — insert before it
                    break;
                }

                result.push(lines[i].to_string());

                if !is_key_line && !is_comment {
                    i += 1;
                    break;
                }
                i += 1;
            }

            // Insert new agent keys here
            for name in new_agents {
                result.push(format!("{} = \"\"", name));
            }
            continue;
        }

        i += 1;
    }

    result.join("\n")
}

fn strip_skills_reference(content: &str) -> String {
    // Remove the skills reference block — try both old and new header formats.
    for marker in [
        "# ── Installed skills (reference)",
        "# Installed skills (for reference",
    ] {
        if let Some(pos) = content.find(marker) {
            let after = &content[pos..];
            let all_comments = after
                .lines()
                .all(|line| line.starts_with('#') || line.trim().is_empty());
            if all_comments {
                return content[..pos].to_string();
            }
        }
    }
    content.to_string()
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct MappingConfig {
    #[serde(rename = "agent-skills")]
    pub agent_skills: HashMap<String, Vec<String>>,
    #[serde(rename = "role-skills")]
    pub role_skills: HashMap<String, Vec<String>>,
    #[serde(rename = "hook-events")]
    pub hook_events: HashMap<String, HookTarget>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum HookTarget {
    All(String),        // "all"
    Roles(Vec<String>), // ["engineer", "reviewer"]
}

impl MappingConfig {
    pub fn load(source_dir: &Path) -> Self {
        let path = source_dir.join("vstack.toml");
        if !path.exists() {
            return Self::default();
        }
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn skills_for_agent(
        &self,
        agent_name: &str,
        agent_role: &crate::agent::AgentRole,
        available: &[String],
    ) -> Vec<String> {
        let mut matched = crate::agent::prefixed_skill_matches(agent_name, available);
        let mut matched_set = HashSet::new();
        let available_set: HashSet<&str> = available.iter().map(|skill| skill.as_str()).collect();
        let name = agent_name.to_lowercase();
        for skill in &matched {
            matched_set.insert(skill.clone());
        }
        let mut push_unique = |skill: &str| {
            if available_set.contains(skill) && matched_set.insert(skill.to_string()) {
                matched.push(skill.to_string());
            }
        };

        // 2. Explicit agent-skills from config
        if let Some(extras) = self.agent_skills.get(&name) {
            for s in extras {
                push_unique(s);
            }
        }
        // Also check without reviewer- prefix for reviewer agents
        if let Some(suffix) = name.strip_prefix("reviewer-")
            && let Some(extras) = self.agent_skills.get(suffix)
        {
            for s in extras {
                push_unique(s);
            }
        }

        // 3. Role-skills from config
        let role_key = agent_role.as_str();
        if let Some(role_skills) = self.role_skills.get(role_key) {
            for s in role_skills {
                push_unique(s);
            }
        }

        matched.sort();
        matched
    }

    pub fn hooks_for_agent<'a>(
        &self,
        agent_role: &crate::agent::AgentRole,
        hooks: &'a [crate::hook::Hook],
    ) -> Vec<&'a crate::hook::Hook> {
        let role_str = agent_role.as_str();

        if self.hook_events.is_empty() {
            // Fallback to old heuristic
            return crate::agent::match_hooks(agent_role, hooks);
        }

        hooks
            .iter()
            .filter(|h| {
                let matcher = h.matcher.as_deref().unwrap_or("");
                let key = format!("{}:{}", h.event, matcher);
                // Try exact key first, then event-only key
                let target = self
                    .hook_events
                    .get(&key)
                    .or_else(|| self.hook_events.get(&format!("{}:", h.event)));

                match target {
                    Some(HookTarget::All(s)) if s == "all" => true,
                    Some(HookTarget::Roles(roles)) => roles.iter().any(|r| r == role_str),
                    _ => false,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentRole;

    #[test]
    fn default_config_falls_back_to_prefix_matching() {
        let config = MappingConfig::default();
        let available = vec![
            "rust-arch".into(),
            "rust-async".into(),
            "python-web".into(),
            "issue-lifecycle".into(),
        ];
        let matched = config.skills_for_agent("rust", &AgentRole::Engineer, &available);
        assert!(matched.contains(&"rust-arch".to_string()));
        assert!(matched.contains(&"rust-async".to_string()));
        assert!(!matched.contains(&"python-web".to_string()));
    }

    #[test]
    fn config_adds_explicit_agent_skills() {
        let mut config = MappingConfig::default();
        config
            .agent_skills
            .insert("iced".into(), vec!["iced-rs".into(), "trading-design".into()]);
        let available = vec!["iced-rs".into(), "trading-design".into(), "other".into()];
        let matched = config.skills_for_agent("iced", &AgentRole::Engineer, &available);
        assert!(matched.contains(&"iced-rs".to_string()));
        assert!(matched.contains(&"trading-design".to_string()));
        assert!(!matched.contains(&"other".to_string()));
    }

    #[test]
    fn config_adds_role_skills() {
        let mut config = MappingConfig::default();
        config
            .role_skills
            .insert("engineer".into(), vec!["github".into(), "worktree".into()]);
        let available = vec!["github".into(), "worktree".into(), "linear".into()];
        let matched = config.skills_for_agent("rust", &AgentRole::Engineer, &available);
        assert!(matched.contains(&"github".to_string()));
        assert!(matched.contains(&"worktree".to_string()));
        assert!(!matched.contains(&"linear".to_string()));
    }

    #[test]
    fn hook_target_all_matches_every_role() {
        let mut config = MappingConfig::default();
        config
            .hook_events
            .insert("PreToolUse:Bash".into(), HookTarget::All("all".into()));

        let hooks = vec![crate::hook::Hook {
            name: "h1".into(),
            event: "PreToolUse".into(),
            matcher: Some("Bash".into()),
            description: "".into(),
            safety: None,
            timeout: None,
            script: "".into(),
            source_path: std::path::PathBuf::new(),
        }];

        assert_eq!(
            config.hooks_for_agent(&AgentRole::Engineer, &hooks).len(),
            1
        );
        assert_eq!(
            config.hooks_for_agent(&AgentRole::Reviewer, &hooks).len(),
            1
        );
        assert_eq!(config.hooks_for_agent(&AgentRole::Manager, &hooks).len(), 1);
    }

    #[test]
    fn hook_target_roles_filters_correctly() {
        let mut config = MappingConfig::default();
        config.hook_events.insert(
            "PostToolUse:Edit|Write".into(),
            HookTarget::Roles(vec!["engineer".into()]),
        );

        let hooks = vec![crate::hook::Hook {
            name: "h2".into(),
            event: "PostToolUse".into(),
            matcher: Some("Edit|Write".into()),
            description: "".into(),
            safety: None,
            timeout: None,
            script: "".into(),
            source_path: std::path::PathBuf::new(),
        }];

        assert_eq!(
            config.hooks_for_agent(&AgentRole::Engineer, &hooks).len(),
            1
        );
        assert_eq!(
            config.hooks_for_agent(&AgentRole::Reviewer, &hooks).len(),
            0
        );
    }

    #[test]
    fn empty_hook_events_falls_back() {
        let config = MappingConfig::default();
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
        // Engineer gets all hooks via the old heuristic
        assert_eq!(
            config.hooks_for_agent(&AgentRole::Engineer, &hooks).len(),
            2
        );
    }

    #[test]
    fn load_missing_file_returns_default() {
        let config = MappingConfig::load(std::path::Path::new("/nonexistent/path"));
        assert!(config.agent_skills.is_empty());
        assert!(config.role_skills.is_empty());
        assert!(config.hook_events.is_empty());
    }

    #[test]
    fn reviewer_agent_checks_stripped_prefix() {
        let mut config = MappingConfig::default();
        config
            .agent_skills
            .insert("iced".into(), vec!["trading-design".into()]);
        let available = vec!["iced-rs".into(), "trading-design".into()];
        let matched = config.skills_for_agent("reviewer-iced", &AgentRole::Reviewer, &available);
        assert!(matched.contains(&"iced-rs".to_string()));
        assert!(matched.contains(&"trading-design".to_string()));
    }

    #[test]
    fn project_config_parses_all_sections() {
        let toml = r#"
[custom-skills]
rust = [
  { name = "my-testing", description = "Custom testing patterns" },
  { name = "my-lint", description = "Custom lint rules" },
]

[agent-guidance]
rust = "Use when working on backend Rust services."

[agent-instructions]
rust = "Always run clippy before committing."
"#;
        let config: ProjectConfig = toml::from_str(toml).unwrap();
        let skills = config.custom_skills_for("rust");
        assert_eq!(skills.len(), 2);
        assert_eq!(skills[0].0, "my-testing");
        assert_eq!(skills[1].1, "Custom lint rules");

        assert_eq!(
            config.guidance_for("rust"),
            Some("Use when working on backend Rust services.")
        );
        assert_eq!(
            config.instructions_for("rust"),
            Some("Always run clippy before committing.")
        );

        // Unknown agent returns empty/None
        assert!(config.custom_skills_for("unknown").is_empty());
        assert!(config.guidance_for("unknown").is_none());
        assert!(config.instructions_for("unknown").is_none());
    }

    #[test]
    fn project_config_missing_file_returns_default() {
        let config = ProjectConfig::load(std::path::Path::new("/nonexistent/path"));
        assert!(config.custom_skills.is_empty());
        assert!(config.agent_guidance.is_empty());
        assert!(config.agent_instructions.is_empty());
    }

    #[test]
    fn project_config_ignores_mapping_sections() {
        // A vstack.toml with both source mapping and project customization sections
        let toml = r#"
[agent-skills]
iced = ["iced-rs"]

[agent-guidance]
rust = "Use for Rust work."
"#;
        let config: ProjectConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.guidance_for("rust"), Some("Use for Rust work."));
        // custom_skills is empty — [agent-skills] is a different section
        assert!(config.custom_skills.is_empty());
    }

    #[test]
    fn update_project_config_appends_new_agents() {
        let dir = std::env::temp_dir().join(format!(
            "vstack_test_update_config_{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("vstack.toml");

        // Create initial config with "rust" agent
        create_project_config(&path, &["rust".into()], &["rust-arch".into()]);
        let initial = std::fs::read_to_string(&path).unwrap();
        assert!(initial.contains("# rust ="));
        assert!(initial.contains("#   rust-arch"));

        // Update with "rust" + new "iced" agent and new skill
        update_project_config(
            &path,
            &["rust".into(), "iced".into()],
            &["rust-arch".into(), "trading-design".into()],
        );
        let updated = std::fs::read_to_string(&path).unwrap();

        // Original rust placeholders preserved
        assert!(updated.contains("# rust ="));
        // New iced agent added (uncommented, empty value)
        assert!(updated.contains("iced = \"\""));
        // Skills reference updated
        assert!(updated.contains("#   trading-design"));
        // Old skills still listed
        assert!(updated.contains("#   rust-arch"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn update_project_config_preserves_user_edits() {
        let dir = std::env::temp_dir().join(format!(
            "vstack_test_preserve_edits_{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("vstack.toml");

        // Simulate user-edited file with active (uncommented) config
        let user_content = r#"[agent-guidance]
rust = "Use for my backend services."

[agent-instructions]
rust = "Always use thiserror for errors."
"#;
        std::fs::write(&path, user_content).unwrap();

        // Update with rust (already present) + new iced
        update_project_config(
            &path,
            &["rust".into(), "iced".into()],
            &["trading-design".into()],
        );
        let updated = std::fs::read_to_string(&path).unwrap();

        // User content preserved
        assert!(updated.contains("rust = \"Use for my backend services.\""));
        assert!(updated.contains("rust = \"Always use thiserror for errors.\""));
        // New agent added (uncommented, empty value)
        assert!(updated.contains("iced = \"\""));
        // Rust not duplicated
        let iced_section = updated.find("iced = \"\"").unwrap();
        assert!(
            !updated[iced_section..].contains("# rust ="),
            "rust should not appear in new agents section"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn update_project_config_no_change_when_all_present() {
        let dir = std::env::temp_dir().join(format!(
            "vstack_test_no_change_{}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("vstack.toml");

        create_project_config(&path, &["rust".into()], &["rust-arch".into()]);
        let before = std::fs::read_to_string(&path).unwrap();

        // Same agents/skills — should not add "New agents" section
        update_project_config(&path, &["rust".into()], &["rust-arch".into()]);
        let after = std::fs::read_to_string(&path).unwrap();

        assert!(!after.contains("── New agents"));
        // Content should be essentially the same (skills ref regenerated but identical)
        assert_eq!(before.trim(), after.trim());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
