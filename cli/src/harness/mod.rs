pub mod claude;
pub mod codex;
pub mod cursor;
pub mod opencode;

use crate::agent::Agent;
use crate::skill::Skill;
use anyhow::{Result, bail};
use std::path::PathBuf;

/// Supported AI coding harnesses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Harness {
    ClaudeCode,
    Cursor,
    OpenCode,
    Codex,
}

impl Harness {
    pub const ALL: &[Harness] = &[
        Harness::ClaudeCode,
        Harness::Cursor,
        Harness::OpenCode,
        Harness::Codex,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Harness::ClaudeCode => "Claude Code",
            Harness::Cursor => "Cursor",
            Harness::OpenCode => "OpenCode",
            Harness::Codex => "Codex",
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Harness::ClaudeCode => "claude-code",
            Harness::Cursor => "cursor",
            Harness::OpenCode => "opencode",
            Harness::Codex => "codex",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "claude-code" | "claude" => Some(Harness::ClaudeCode),
            "cursor" => Some(Harness::Cursor),
            "opencode" => Some(Harness::OpenCode),
            "codex" => Some(Harness::Codex),
            _ => None,
        }
    }

    pub fn supports_global_scope(&self) -> bool {
        !matches!(self, Harness::Cursor)
    }

    pub fn global_support_reason(&self) -> Option<&'static str> {
        match self {
            Harness::Cursor => {
                Some("Cursor user rules are configured in settings, not a global rules directory.")
            }
            _ => None,
        }
    }

    /// Directory for agents relative to project/home root
    pub fn agents_dir(&self, global: bool) -> PathBuf {
        match self {
            Harness::ClaudeCode => {
                if global {
                    crate::config::claude_global_dir().join("agents")
                } else {
                    crate::config::project_root().join(".claude").join("agents")
                }
            }
            Harness::Cursor => {
                if global {
                    crate::config::cursor_global_dir().join("rules")
                } else {
                    crate::config::project_root().join(".cursor").join("rules")
                }
            }
            Harness::OpenCode => {
                if global {
                    crate::config::opencode_global_dir().join("agents")
                } else {
                    crate::config::project_root()
                        .join(".opencode")
                        .join("agents")
                }
            }
            Harness::Codex => {
                if global {
                    crate::config::codex_home_dir().join("agents")
                } else {
                    crate::config::project_root().join(".codex").join("agents")
                }
            }
        }
    }

    /// Directory for skills relative to project/home root
    pub fn skills_dir(&self, global: bool) -> PathBuf {
        match self {
            Harness::ClaudeCode => {
                if global {
                    crate::config::claude_global_dir().join("skills")
                } else {
                    crate::config::project_root().join(".claude").join("skills")
                }
            }
            Harness::Cursor => {
                if global {
                    crate::config::cursor_global_dir().join("rules")
                } else {
                    crate::config::project_root().join(".cursor").join("rules")
                }
            }
            Harness::OpenCode => {
                if global {
                    crate::config::opencode_global_dir().join("skills")
                } else {
                    crate::config::project_root()
                        .join(".opencode")
                        .join("skills")
                }
            }
            Harness::Codex => {
                if global {
                    crate::config::codex_home_dir().join("skills")
                } else {
                    crate::config::project_root().join(".agents").join("skills")
                }
            }
        }
    }

    pub fn hooks_dir(&self, global: bool) -> Option<PathBuf> {
        match self {
            Harness::ClaudeCode => Some(if global {
                crate::config::claude_global_dir().join("hooks")
            } else {
                crate::config::project_root().join(".claude").join("hooks")
            }),
            _ => None,
        }
    }

    pub fn install_root(&self, global: bool) -> PathBuf {
        match self {
            Harness::ClaudeCode => {
                if global {
                    crate::config::claude_global_dir()
                } else {
                    crate::config::project_root().join(".claude")
                }
            }
            Harness::Cursor => {
                if global {
                    crate::config::cursor_global_dir()
                } else {
                    crate::config::project_root().join(".cursor")
                }
            }
            Harness::OpenCode => {
                if global {
                    crate::config::opencode_global_dir()
                } else {
                    crate::config::project_root().join(".opencode")
                }
            }
            Harness::Codex => {
                if global {
                    crate::config::codex_home_dir()
                } else {
                    crate::config::project_root().join(".codex")
                }
            }
        }
    }

    pub fn summary_paths(&self, global: bool) -> Vec<PathBuf> {
        let mut paths = vec![self.install_root(global)];
        if matches!(self, Harness::OpenCode) {
            let config_path = if global {
                crate::config::opencode_global_config_path()
            } else {
                crate::config::opencode_project_config_path()
            };
            if !paths.contains(&config_path) {
                paths.push(config_path);
            }
        }
        paths
    }

    /// Generate a harness-specific agent file and return the output path
    pub fn generate_agent(
        &self,
        agent: &Agent,
        global: bool,
        skills: &[(String, String)],
        hooks: &[crate::hook::Hook],
    ) -> Result<PathBuf> {
        if global && !self.supports_global_scope() {
            bail!(
                "{}",
                self.global_support_reason()
                    .unwrap_or("Global scope is unsupported")
            );
        }
        let dir = self.agents_dir(global);
        match self {
            Harness::ClaudeCode => claude::generate_agent(agent, &dir, skills, hooks),
            Harness::Cursor => cursor::generate_agent(agent, &dir, skills, hooks),
            Harness::OpenCode => opencode::generate_agent(agent, &dir, skills, hooks),
            Harness::Codex => codex::generate_agent(agent, &dir, skills, hooks),
        }
    }

    /// Install a skill directory to the harness-specific location
    pub fn install_skill(&self, skill: &Skill, global: bool) -> Result<PathBuf> {
        if global && !self.supports_global_scope() {
            bail!(
                "{}",
                self.global_support_reason()
                    .unwrap_or("Global scope is unsupported")
            );
        }
        let dest = self.skills_dir(global).join(&skill.name);
        Ok(dest)
    }

    /// Check if this harness is detected on the system
    pub fn is_detected(&self) -> bool {
        let project = crate::config::project_root();
        match self {
            Harness::ClaudeCode => crate::config::claude_global_dir().exists(),
            Harness::Cursor => crate::config::cursor_global_dir().exists(),
            Harness::OpenCode => {
                crate::config::opencode_global_dir().exists()
                    || crate::config::opencode_global_config_path().exists()
                    || project.join("opencode.json").exists()
                    || project.join("opencode.jsonc").exists()
            }
            Harness::Codex => crate::config::codex_home_dir().exists(),
        }
    }
}

impl std::fmt::Display for Harness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::Harness;

    #[test]
    fn cursor_is_project_scope_only() {
        assert!(!Harness::Cursor.supports_global_scope());
        assert!(Harness::ClaudeCode.supports_global_scope());
        assert!(Harness::OpenCode.supports_global_scope());
        assert!(Harness::Codex.supports_global_scope());
    }
}
