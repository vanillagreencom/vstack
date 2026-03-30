use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

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
}
