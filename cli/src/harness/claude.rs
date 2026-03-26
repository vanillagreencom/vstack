use crate::agent::{self, Agent};
use crate::hook::Hook;
use anyhow::Result;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Generate a Claude Code agent file (.claude/agents/<name>.md)
///
/// Format: YAML frontmatter with name, description, model, color, skills, hooks
/// followed by markdown body.
pub fn generate_agent(
    agent: &Agent,
    dir: &Path,
    skills: &[(String, String)],
    hooks: &[Hook],
) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)?;

    let path = dir.join(format!("{}.md", agent.name));

    let mut output = String::new();
    output.push_str("---\n");
    output.push_str(&format!("name: {}\n", agent.name));
    output.push_str(&format!("description: {}\n", agent.description));

    // Map model to Claude Code format
    let model = agent.model_id("claude-code");
    output.push_str(&format!("model: {}\n", model));

    if let Some(ref color) = agent.color {
        output.push_str(&format!("color: {}\n", color));
    }

    // Skills frontmatter
    if !skills.is_empty() {
        let names: Vec<&str> = skills.iter().map(|(n, _)| n.as_str()).collect();
        output.push_str(&format!("skills: {}\n", names.join(", ")));
    }

    // Hooks frontmatter (Claude Code native format)
    if !hooks.is_empty() {
        output.push_str(&format_hooks_yaml(hooks));
    }

    output.push_str("---\n\n");

    // Insert "Load These Skills" after first heading's intro
    let skills_section = agent::load_skills_section(skills);
    let body = agent::insert_after_intro(&agent.body, &skills_section);
    output.push_str(&body);

    if !output.ends_with('\n') {
        output.push('\n');
    }

    std::fs::write(&path, &output)?;
    Ok(path)
}

/// Format hooks into Claude Code YAML frontmatter format.
///
/// Groups hooks by event, then by matcher:
/// ```yaml
/// hooks:
///   PreToolUse:
///     - matcher: Bash
///       hooks:
///         - type: command
///           command: ".claude/hooks/block-bare-cd.sh"
/// ```
fn format_hooks_yaml(hooks: &[Hook]) -> String {
    // Group by event → matcher → list of hooks
    let mut by_event: BTreeMap<&str, BTreeMap<Option<&str>, Vec<&Hook>>> = BTreeMap::new();

    for hook in hooks {
        by_event
            .entry(&hook.event)
            .or_default()
            .entry(hook.matcher.as_deref())
            .or_default()
            .push(hook);
    }

    let mut yaml = String::from("hooks:\n");

    for (event, matchers) in &by_event {
        yaml.push_str(&format!("  {}:\n", event));
        for (matcher, hook_list) in matchers {
            if let Some(m) = matcher {
                yaml.push_str(&format!("    - matcher: {}\n", m));
            } else {
                yaml.push_str("    - matcher: \"*\"\n");
            }
            yaml.push_str("      hooks:\n");
            for h in hook_list {
                yaml.push_str("        - type: command\n");
                yaml.push_str(&format!(
                    "          command: \".claude/hooks/{}.sh\"\n",
                    h.name
                ));
            }
        }
    }

    yaml
}
