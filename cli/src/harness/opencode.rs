use crate::agent::{self, Agent, AgentRole};
use crate::hook::Hook;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Generate an OpenCode agent as a markdown file in `.opencode/agents/<name>.md`.
///
/// Format: YAML frontmatter (description, mode, model, permission/tools)
/// followed by the agent body as the system prompt.
pub fn generate_agent(
    agent: &Agent,
    dir: &Path,
    skills: &[(String, String)],
    _hooks: &[Hook],
) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)?;

    let path = dir.join(format!("{}.md", agent.name));

    // Determine mode based on role
    let mode = match agent.role {
        AgentRole::Engineer => "primary",
        AgentRole::Reviewer => "subagent",
        AgentRole::Manager => "subagent",
    };

    let model = agent.model_id("openai");

    let mut output = String::new();
    output.push_str("---\n");
    output.push_str(&format!("description: {}\n", yaml_str(&agent.description)));
    output.push_str(&format!("mode: {mode}\n"));
    output.push_str(&format!("model: {model}\n"));

    // Permission/tools based on role
    match agent.role {
        AgentRole::Reviewer => {
            output.push_str("permission:\n");
            output.push_str("  edit: deny\n");
            output.push_str("  bash:\n");
            output.push_str("    \"*\": ask\n");
            output.push_str("    \"git diff*\": allow\n");
            output.push_str("    \"git log*\": allow\n");
            output.push_str("    \"grep *\": allow\n");
        }
        AgentRole::Engineer => {
            // Full access — no restrictions needed
        }
        AgentRole::Manager => {
            output.push_str("permission:\n");
            output.push_str("  edit: deny\n");
            output.push_str("  bash:\n");
            output.push_str("    \"*\": ask\n");
        }
    }

    output.push_str("---\n\n");

    let skills_section = agent::load_skills_section(skills);
    let body = agent::insert_after_intro(&agent.body, &skills_section);
    output.push_str(&body);

    if !output.ends_with('\n') {
        output.push('\n');
    }

    std::fs::write(&path, &output)?;
    Ok(path)
}

/// Escape a YAML string value — quote if it contains special characters
fn yaml_str(s: &str) -> String {
    if s.contains(':') || s.contains('#') || s.contains('"') || s.contains('\'') || s.contains('\n')
    {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s.to_string()
    }
}
