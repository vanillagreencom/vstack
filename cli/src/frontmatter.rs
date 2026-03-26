use anyhow::{Context, Result};

pub fn split_yaml_frontmatter(content: &str) -> Result<(String, String)> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        anyhow::bail!("missing YAML frontmatter (expected --- delimiter)");
    }

    let after_first = &trimmed[3..];
    let close = after_first
        .find("\n---")
        .context("missing closing --- delimiter")?;
    let frontmatter = after_first[..close].trim().to_string();
    let body = after_first[close + 4..].trim().to_string();

    Ok((frontmatter, body))
}
