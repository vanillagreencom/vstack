use crate::agent::{self, Agent};
use crate::hook::Hook;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Generate a Cursor rule file (.cursor/rules/<name>.mdc)
///
/// Format: YAML frontmatter with description, alwaysApply
/// followed by markdown body content.
pub fn generate_agent(
    agent: &Agent,
    dir: &Path,
    skills: &[(String, String)],
    _hooks: &[Hook],
) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)?;

    let path = dir.join(format!("{}.mdc", agent.name));

    let mut output = String::new();
    output.push_str("---\n");
    output.push_str(&format!(
        "description: \"{} — {}\"\n",
        agent.name, agent.description
    ));
    output.push_str("alwaysApply: false\n");
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
