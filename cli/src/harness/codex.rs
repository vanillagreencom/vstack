use crate::agent::{self, Agent, AgentRole};
use crate::hook::Hook;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Generate a Codex agent file (.codex/agents/<name>.toml)
///
/// Format: TOML with name, description, model, sandbox_mode,
/// and developer_instructions (the agent body).
pub fn generate_agent(
    agent: &Agent,
    dir: &Path,
    skills: &[(String, String)],
    _hooks: &[Hook],
    extras: &agent::AgentExtras,
) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)?;

    let path = dir.join(format!("{}.toml", agent.name));

    // Map role to sandbox_mode
    let sandbox_mode = match agent.role {
        AgentRole::Reviewer => "read-only",
        AgentRole::Engineer => "danger-full-access",
        AgentRole::Manager => "danger-full-access",
    };

    // Map model to reasoning effort
    let lower = agent.model.to_lowercase();
    let (model, reasoning_effort) = match lower.as_str() {
        "opus" => ("gpt-5.4", "xhigh"),
        "sonnet" => ("gpt-5.4", "high"),
        "haiku" => ("gpt-5.4", "medium"),
        other => (other, "high"),
    };

    // Build TOML manually to control format (triple-quoted developer_instructions)
    let mut output = String::new();
    output.push_str(&format!("name = \"{}\"\n", escape_toml(&agent.name)));
    output.push_str(&format!(
        "description = \"{}\"\n",
        escape_toml(&agent.description)
    ));
    output.push_str(&format!("model = \"{model}\"\n"));
    output.push_str(&format!(
        "model_reasoning_effort = \"{reasoning_effort}\"\n"
    ));
    output.push_str(&format!("sandbox_mode = \"{sandbox_mode}\"\n"));

    // Developer instructions as multiline TOML string
    output.push_str("developer_instructions = '''\n");

    let guidance = agent::guidance_section(extras.guidance.as_deref());
    let skills_section = agent::load_skills_section(skills);
    let combined = format!("{}{}", guidance, skills_section);
    let body = agent::insert_after_intro(&agent.body, &combined);
    let hooks_prose = agent::custom_hooks_section(&extras.custom_hooks);
    let instructions = agent::instructions_section(extras.instructions.as_deref());
    let body = agent::append_section(&body, &hooks_prose);
    let body = agent::append_section(&body, &instructions);
    output.push_str(&body);

    if !output.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("'''\n");

    std::fs::write(&path, &output)?;
    Ok(path)
}

fn escape_toml(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
