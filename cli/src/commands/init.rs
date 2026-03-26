use anyhow::Result;
use std::path::Path;

pub fn run(name: Option<&str>) -> Result<()> {
    let name = match name {
        Some(n) => n.to_string(),
        None => {
            eprintln!("Usage: vstack init <name>");
            return Ok(());
        }
    };

    // Ask what to create
    eprintln!("Creating new item: {name}\n");

    let agent_path = Path::new("agents").join(format!("{name}.md"));
    let skill_dir = Path::new("skills").join(&name);

    if agent_path.exists() {
        eprintln!("Agent already exists: {}", agent_path.display());
    } else {
        // Create agent template
        std::fs::create_dir_all("agents")?;
        let template = format!(
            r#"---
name: {name}
description: TODO - describe when to use this agent
model: sonnet
role: engineer
color: green
---

# {title}

TODO - describe what this agent does.

## Capabilities

- TODO

## Guidelines

- TODO
"#,
            name = name,
            title = title_case(&name),
        );
        std::fs::write(&agent_path, template)?;
        eprintln!("Created agent: {}", agent_path.display());
    }

    if skill_dir.exists() {
        eprintln!("Skill directory already exists: {}", skill_dir.display());
    } else {
        // Create skill template
        std::fs::create_dir_all(&skill_dir)?;
        let skill_md = format!(
            r#"---
name: {name}
description: TODO - describe this skill
license: MIT
---

# {title}

TODO - skill instructions.
"#,
            name = name,
            title = title_case(&name),
        );
        std::fs::write(skill_dir.join("SKILL.md"), skill_md)?;
        eprintln!("Created skill: {}/SKILL.md", skill_dir.display());
    }

    Ok(())
}

fn title_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
