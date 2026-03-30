use crate::agent::Agent;
use crate::config::{InstallMethod, ItemKind, LockEntry, LockFile};
use crate::harness::Harness;
use crate::hook::Hook;
use crate::skill::Skill;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Result of a single installation
pub struct InstallResult {
    pub name: String,
    pub kind: ItemKind,
    pub harness: Harness,
    pub path: PathBuf,
    pub detail: String,
}

/// Install an agent to a specific harness
pub fn install_agent(
    agent: &Agent,
    harness: Harness,
    global: bool,
    skills: &[(String, String)],
    hooks: &[crate::hook::Hook],
    extras: &crate::agent::AgentExtras,
) -> Result<InstallResult> {
    let output_path = harness.generate_agent(agent, global, skills, hooks, extras)?;

    let detail = format!(
        "{} → {} ({})",
        agent.name,
        output_path.display(),
        harness.name()
    );

    Ok(InstallResult {
        name: agent.name.clone(),
        kind: ItemKind::Agent,
        harness,
        detail,
        path: output_path,
    })
}

/// Install a skill directory to a specific harness.
///
/// Symlink mode: copy to a canonical dir (`.agents/skills/<name>/`) within the
/// project, then symlink from each harness-specific dir to the canonical copy.
/// All paths stay within the project root — no external symlinks.
///
/// Copy mode: copy directly to each harness dir.
pub fn install_skill(
    skill: &Skill,
    harness: Harness,
    global: bool,
    method: InstallMethod,
) -> Result<InstallResult> {
    let dest = harness.install_skill(skill, global)?;

    // Canonical location: .agents/skills/<name>/ (universal, like Vercel npx skills)
    let canonical = if global && matches!(harness, Harness::Codex) {
        crate::config::codex_home_dir()
            .join("skills")
            .join(&skill.name)
    } else if global {
        crate::config::global_state_dir()
            .join("skills")
            .join(&skill.name)
    } else {
        crate::config::project_root()
            .join(".agents")
            .join("skills")
            .join(&skill.name)
    };

    let detail = match method {
        InstallMethod::Symlink => {
            // Step 1: Copy to canonical location (always refresh from source)
            remove_existing(&canonical)?;
            copy_dir(&skill.source_dir, &canonical)?;

            // Step 2: If this harness IS the canonical path, we're done
            if dest == canonical {
                format!(
                    "{} → {} (canonical, {})",
                    skill.name,
                    canonical.display(),
                    harness.name()
                )
            } else {
                // Step 3: Symlink from harness dir to canonical
                if let Some(parent) = dest.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                remove_existing(&dest)?;

                let rel = relative_path(dest.parent().unwrap(), &canonical)?;
                #[cfg(unix)]
                std::os::unix::fs::symlink(&rel, &dest).with_context(|| {
                    format!("symlinking {} → {}", dest.display(), rel.display())
                })?;

                #[cfg(not(unix))]
                copy_dir(&canonical, &dest)?;

                format!(
                    "{} → {} (symlink, {})",
                    skill.name,
                    dest.display(),
                    harness.name()
                )
            }
        }
        InstallMethod::Copy => {
            remove_existing(&dest)?;
            copy_dir(&skill.source_dir, &dest)?;
            format!(
                "{} → {} (copy, {})",
                skill.name,
                dest.display(),
                harness.name()
            )
        }
    };

    Ok(InstallResult {
        name: skill.name.clone(),
        kind: ItemKind::Skill,
        harness,
        path: dest,
        detail,
    })
}

/// Install a hook to a specific harness.
///
/// - Claude Code: copy script + add to settings.json hooks
/// - OpenCode: add permission rules to opencode.json
/// - Codex: append safety prose to all agent TOML developer_instructions
/// - Cursor: append safety advisory to all .mdc rule files
pub fn install_hook(
    hook: &Hook,
    harness: Harness,
    global: bool,
    agents: &[Agent],
) -> Result<String> {
    match harness {
        Harness::ClaudeCode => install_hook_claude(hook, global)?,
        Harness::OpenCode => install_hook_opencode(hook, global)?,
        Harness::Codex => install_hook_codex(hook, global, agents)?,
        Harness::Cursor => install_hook_cursor(hook, global)?,
    }

    Ok(format!(
        "[hook] {} → {} ({})",
        hook.name,
        harness.name(),
        hook.event
    ))
}

/// Claude Code: copy hook script + merge into settings.json
fn install_hook_claude(hook: &Hook, global: bool) -> Result<()> {
    // Copy the script
    let hooks_dir = Harness::ClaudeCode
        .hooks_dir(global)
        .expect("Claude hooks dir");
    std::fs::create_dir_all(&hooks_dir)?;
    let dest = hooks_dir.join(format!("{}.sh", hook.name));
    std::fs::write(&dest, &hook.script)?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;
    }

    // Merge into settings.json
    let settings_path = if global {
        crate::config::claude_global_dir().join("settings.json")
    } else {
        crate::config::project_root()
            .join(".claude")
            .join("settings.json")
    };
    let mut settings: serde_json::Value = if settings_path.exists() {
        let content = std::fs::read_to_string(&settings_path)?;
        serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    let map = settings.as_object_mut().unwrap();
    if !map.contains_key("hooks") {
        map.insert("hooks".into(), serde_json::json!({}));
    }
    let hooks_obj = map.get_mut("hooks").unwrap().as_object_mut().unwrap();

    // Build the hook entry
    let hook_entry = {
        let mut entry = serde_json::json!({
            "hooks": [{
                "type": "command",
                "command": format!(".claude/hooks/{}.sh", hook.name)
            }]
        });
        if let Some(ref matcher) = hook.matcher {
            entry
                .as_object_mut()
                .unwrap()
                .insert("matcher".into(), serde_json::Value::String(matcher.clone()));
        }
        if let Some(timeout) = hook.timeout {
            entry
                .as_object_mut()
                .unwrap()
                .insert("timeout".into(), serde_json::Value::Number(timeout.into()));
        }
        entry
    };

    // Add to the appropriate event array
    if !hooks_obj.contains_key(&hook.event) {
        hooks_obj.insert(hook.event.clone(), serde_json::json!([]));
    }
    let event_arr = hooks_obj
        .get_mut(&hook.event)
        .unwrap()
        .as_array_mut()
        .unwrap();

    // Don't duplicate if already present
    let already_exists = event_arr.iter().any(|e| {
        e.get("hooks")
            .and_then(|h| h.as_array())
            .and_then(|a| a.first())
            .and_then(|h| h.get("command"))
            .and_then(|c| c.as_str())
            .is_some_and(|c| c.contains(&hook.name))
    });

    if !already_exists {
        event_arr.push(hook_entry);
    }

    let output = serde_json::to_string_pretty(&settings)?;
    std::fs::write(&settings_path, output)?;

    Ok(())
}

/// OpenCode: add permission rules based on hook intent
fn install_hook_opencode(hook: &Hook, global: bool) -> Result<()> {
    let config_path = if global {
        crate::config::opencode_global_config_path()
    } else {
        crate::config::opencode_project_config_path()
    };
    let instruction_path = opencode_hook_instruction_path(global, &hook.name);
    let instruction_ref = opencode_hook_instruction_ref(global, &hook.name);
    install_hook_opencode_at_path(hook, &config_path, &instruction_path, &instruction_ref)
}

fn opencode_hook_instruction_path(global: bool, name: &str) -> PathBuf {
    let file_name = format!("vstack-hook-{name}.md");
    if global {
        crate::config::opencode_global_dir()
            .join("instructions")
            .join(file_name)
    } else {
        crate::config::project_root()
            .join(".opencode")
            .join("instructions")
            .join(file_name)
    }
}

fn opencode_hook_instruction_ref(global: bool, name: &str) -> String {
    let file_name = format!("vstack-hook-{name}.md");
    if global {
        format!("instructions/{file_name}")
    } else {
        format!(".opencode/instructions/{file_name}")
    }
}

fn install_hook_opencode_at_path(
    hook: &Hook,
    config_path: &Path,
    instruction_path: &Path,
    instruction_ref: &str,
) -> Result<()> {
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if let Some(parent) = instruction_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let instruction_contents = format!("# Safety: {}\n\n{}", hook.name, hook.safety_prose());
    std::fs::write(instruction_path, instruction_contents)?;

    let mut config: serde_json::Value = if config_path.exists() {
        let content = std::fs::read_to_string(config_path)?;
        serde_json::from_str(&content)?
    } else {
        serde_json::json!({ "$schema": "https://opencode.ai/config.json" })
    };

    let map = config.as_object_mut().unwrap();

    // OpenCode doesn't have hooks — convert to permission rules and instructions
    if !map.contains_key("permission") {
        map.insert("permission".into(), serde_json::json!({}));
    }

    // Add safety-relevant permission restrictions based on hook type
    if hook.event == "PreToolUse" {
        let perms = map.get_mut("permission").unwrap().as_object_mut().unwrap();

        if hook.matcher.as_deref() == Some("Bash") {
            // For bash hooks: set bash permission to "ask" (require confirmation)
            if !perms.contains_key("bash") {
                perms.insert("bash".into(), serde_json::json!({ "*": "ask" }));
            }
        }
    }

    // OpenCode instructions are file paths, so write a dedicated file and reference it.
    if !map.contains_key("instructions") {
        map.insert("instructions".into(), serde_json::json!([]));
    }
    let instructions = map.get_mut("instructions").unwrap().as_array_mut().unwrap();

    let already_has = instructions
        .iter()
        .any(|i| i.as_str() == Some(instruction_ref));

    if !already_has {
        instructions.push(serde_json::Value::String(instruction_ref.to_string()));
    }

    let output = serde_json::to_string_pretty(&config)?;
    std::fs::write(config_path, output)?;

    Ok(())
}

/// Codex: append safety prose to agent TOML developer_instructions
fn install_hook_codex(hook: &Hook, global: bool, agents: &[Agent]) -> Result<()> {
    let agents_dir = Harness::Codex.agents_dir(global);
    if !agents_dir.exists() {
        return Ok(());
    }

    let safety = hook.safety_prose();

    for agent in agents {
        let toml_path = agents_dir.join(format!("{}.toml", agent.name));
        if !toml_path.exists() {
            continue;
        }

        let content = std::fs::read_to_string(&toml_path)?;

        // Check if this hook's safety prose is already embedded
        if content.contains(&hook.name) {
            continue;
        }

        // Find the developer_instructions closing ''' and insert before it
        if let Some(close_pos) = content.rfind("'''") {
            let mut new_content = content[..close_pos].to_string();
            new_content.push_str(&format!("\n## Safety: {}\n\n{}\n", hook.name, safety));
            new_content.push_str(&content[close_pos..]);
            std::fs::write(&toml_path, new_content)?;
        }
    }

    Ok(())
}

/// Cursor: add safety advisory to a dedicated .mdc file
fn install_hook_cursor(hook: &Hook, global: bool) -> Result<()> {
    let rules_dir = Harness::Cursor.agents_dir(global);
    std::fs::create_dir_all(&rules_dir)?;

    let path = rules_dir.join(format!("safety-{}.mdc", hook.name));

    let mut output = String::new();
    output.push_str("---\n");
    output.push_str(&format!(
        "description: \"Safety: {} — {}\"\n",
        hook.name, hook.description
    ));
    output.push_str("alwaysApply: true\n");
    output.push_str("---\n\n");
    output.push_str(&format!("# Safety: {}\n\n", hook.name));
    output.push_str(&hook.safety_prose());

    std::fs::write(&path, output)?;
    Ok(())
}

/// Remove an installed item.
/// Each harness cleanup is independent — one failure doesn't block others.
pub fn remove_item(name: &str, harnesses: &[Harness], global: bool) -> Result<Vec<PathBuf>> {
    let mut removed = Vec::new();

    for harness in harnesses {
        // Agent files
        let agent_paths = match harness {
            Harness::ClaudeCode => vec![harness.agents_dir(global).join(format!("{name}.md"))],
            Harness::Cursor => {
                vec![
                    harness.agents_dir(global).join(format!("{name}.mdc")),
                    harness
                        .agents_dir(global)
                        .join(format!("safety-{name}.mdc")),
                ]
            }
            Harness::OpenCode => vec![harness.agents_dir(global).join(format!("{name}.md"))],
            Harness::Codex => vec![harness.agents_dir(global).join(format!("{name}.toml"))],
        };

        for path in agent_paths {
            if path.exists() && std::fs::remove_file(&path).is_ok() {
                removed.push(path);
            }
        }

        // Skill directories
        let skill_path = harness.skills_dir(global).join(name);
        if skill_path.exists() || skill_path.is_symlink() {
            let ok = if skill_path.is_symlink() || skill_path.is_file() {
                std::fs::remove_file(&skill_path).is_ok()
            } else {
                std::fs::remove_dir_all(&skill_path).is_ok()
            };
            if ok {
                removed.push(skill_path);
            }
        }

        // Hook cleanup (per-harness, each independent)
        if *harness == Harness::ClaudeCode {
            let hook_path = harness
                .hooks_dir(global)
                .expect("Claude hooks dir")
                .join(format!("{name}.sh"));
            if hook_path.exists() && std::fs::remove_file(&hook_path).is_ok() {
                removed.push(hook_path);
            }
            let _ = remove_hook_from_claude_settings(global, name);
        }

        if *harness == Harness::OpenCode {
            let _ = remove_hook_from_opencode_json(global, name);
        }
    }

    let canonical_skill_paths = if global {
        vec![
            crate::config::global_state_dir().join("skills").join(name),
            crate::config::codex_home_dir().join("skills").join(name),
        ]
    } else {
        vec![
            crate::config::project_root()
                .join(".agents")
                .join("skills")
                .join(name),
        ]
    };

    for path in canonical_skill_paths {
        if path.exists() || path.is_symlink() {
            let ok = if path.is_symlink() || path.is_file() {
                std::fs::remove_file(&path).is_ok()
            } else {
                std::fs::remove_dir_all(&path).is_ok()
            };
            if ok {
                removed.push(path);
            }
        }
    }

    Ok(removed)
}

/// Remove a hook entry from Claude Code settings.json
fn remove_hook_from_claude_settings(global: bool, name: &str) -> Result<()> {
    let settings_path = if global {
        crate::config::claude_global_dir().join("settings.json")
    } else {
        crate::config::project_root()
            .join(".claude")
            .join("settings.json")
    };
    if !settings_path.exists() {
        return Ok(());
    }
    let content = std::fs::read_to_string(&settings_path)?;
    let mut settings: serde_json::Value = serde_json::from_str(&content)?;

    let mut changed = false;
    if let Some(hooks) = settings.get_mut("hooks").and_then(|h| h.as_object_mut()) {
        for (_event, entries) in hooks.iter_mut() {
            if let Some(arr) = entries.as_array_mut() {
                let before = arr.len();
                arr.retain(|entry| {
                    !entry
                        .get("hooks")
                        .and_then(|h| h.as_array())
                        .and_then(|a| a.first())
                        .and_then(|h| h.get("command"))
                        .and_then(|c| c.as_str())
                        .is_some_and(|c| c.contains(name))
                });
                if arr.len() != before {
                    changed = true;
                }
            }
        }
    }

    if changed {
        let output = serde_json::to_string_pretty(&settings)?;
        std::fs::write(&settings_path, output)?;
    }
    Ok(())
}

/// Remove hook instructions and permission entries from OpenCode opencode.json
fn remove_hook_from_opencode_json(global: bool, name: &str) -> Result<()> {
    let config_path = if global {
        crate::config::opencode_global_config_path()
    } else {
        crate::config::opencode_project_config_path()
    };
    let instruction_path = opencode_hook_instruction_path(global, name);
    let instruction_ref = opencode_hook_instruction_ref(global, name);
    remove_hook_from_opencode_json_at_path(&config_path, &instruction_path, &instruction_ref, name)
}

fn remove_hook_from_opencode_json_at_path(
    config_path: &Path,
    instruction_path: &Path,
    instruction_ref: &str,
    name: &str,
) -> Result<()> {
    if !config_path.exists() {
        let _ = std::fs::remove_file(instruction_path);
        return Ok(());
    }
    let content = std::fs::read_to_string(config_path)?;
    let mut config: serde_json::Value = serde_json::from_str(&content)?;

    let mut changed = false;

    // Remove the current file-path based format plus the legacy inline prose format.
    let keywords: Vec<&str> = name.split('-').collect();
    if let Some(instructions) = config
        .get_mut("instructions")
        .and_then(|i| i.as_array_mut())
    {
        let before = instructions.len();
        instructions.retain(|i| {
            let Some(s) = i.as_str() else { return true };
            if s == instruction_ref {
                return false;
            }
            let s_lower = s.to_lowercase();
            !keywords.iter().all(|kw| s_lower.contains(kw))
        });
        if instructions.len() != before {
            changed = true;
        }
    }

    let _ = std::fs::remove_file(instruction_path);

    // If no vstack hook instructions remain, remove the temporary bash restriction we added.
    if let Some(map) = config.as_object_mut() {
        let no_vstack_hook_instructions = map
            .get("instructions")
            .and_then(|i| i.as_array())
            .is_none_or(|entries| {
                !entries.iter().any(|entry| {
                    entry
                        .as_str()
                        .is_some_and(|value| value.contains("vstack-hook-"))
                })
            });

        if let Some(instructions) = map.get("instructions").and_then(|i| i.as_array())
            && instructions.is_empty()
        {
            map.remove("instructions");
            changed = true;
        }

        if no_vstack_hook_instructions
            && let Some(permission) = map.get_mut("permission").and_then(|p| p.as_object_mut())
        {
            let remove_bash = permission
                .get("bash")
                .and_then(|bash| bash.as_object())
                .is_some_and(|bash| {
                    bash.len() == 1
                        && bash
                            .get("*")
                            .and_then(|value| value.as_str())
                            .is_some_and(|value| value == "ask")
                });
            if remove_bash {
                permission.remove("bash");
                changed = true;
            }
            if permission.is_empty() {
                map.remove("permission");
                changed = true;
            }
        }
    }

    if changed {
        let output = serde_json::to_string_pretty(&config)?;
        std::fs::write(config_path, output)?;
    }
    Ok(())
}

/// Record installation in lock file
pub fn record_install(
    lock: &mut LockFile,
    results: &[InstallResult],
    source: &str,
    method: InstallMethod,
) {
    for result in results {
        let harness_id = result.harness.id().to_string();
        if let Some(existing) = lock.entries.get_mut(&result.name) {
            if !existing.harnesses.contains(&harness_id) {
                existing.harnesses.push(harness_id);
            }
        } else {
            lock.add(LockEntry {
                name: result.name.clone(),
                kind: result.kind,
                source: source.into(),
                harnesses: vec![harness_id],
                method,
                installed_at: crate::config::now_iso(),
            });
        }
    }
}

/// Compute relative path from `from` to `to`
fn remove_existing(path: &Path) -> Result<()> {
    if path.is_symlink() {
        std::fs::remove_file(path)?;
    } else if path.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

fn normalize_absolute_path(path: &Path) -> PathBuf {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    };

    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

fn relative_path(from: &Path, to: &Path) -> Result<PathBuf> {
    let from_lexical = normalize_absolute_path(from);
    let from_canonical = std::fs::canonicalize(from).unwrap_or_else(|_| from_lexical.clone());
    let to = std::fs::canonicalize(to).unwrap_or_else(|_| normalize_absolute_path(to));

    // If the apparent parent path differs from the real containing directory
    // (for example because an ancestor is a symlink), prefer an absolute
    // target over a confusing relative path that is computed from the real path.
    if from_canonical != from_lexical {
        return Ok(to);
    }

    let from_parts: Vec<_> = from_lexical.components().collect();
    let to_parts: Vec<_> = to.components().collect();

    let common = from_parts
        .iter()
        .zip(to_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let mut rel = PathBuf::new();
    for _ in common..from_parts.len() {
        rel.push("..");
    }
    for part in &to_parts[common..] {
        rel.push(part);
    }

    Ok(rel)
}

/// Recursively copy a directory
fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in walkdir::WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let rel = entry.path().strip_prefix(src)?;
        let target = dst.join(rel);

        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_hook_from_opencode_removes_instruction() {
        let base = std::env::temp_dir().join("vstack_test_opencode");
        let _ = std::fs::create_dir_all(&base);
        let config_path = base.join("opencode.json");
        let instruction_path = base
            .join(".opencode")
            .join("instructions")
            .join("vstack-hook-block-bare-cd.md");
        std::fs::create_dir_all(instruction_path.parent().unwrap()).unwrap();
        std::fs::write(&instruction_path, "# Safety").unwrap();

        let content = r#"{
  "$schema": "https://opencode.ai/config.json",
  "instructions": [
    ".opencode/instructions/vstack-hook-block-bare-cd.md"
  ],
  "permission": {
    "bash": {
      "*": "ask"
    }
  }
}"#;
        std::fs::write(&config_path, content).unwrap();

        remove_hook_from_opencode_json_at_path(
            &config_path,
            &instruction_path,
            ".opencode/instructions/vstack-hook-block-bare-cd.md",
            "block-bare-cd",
        )
        .unwrap();

        let result = std::fs::read_to_string(&config_path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        // instructions and permission should be gone
        assert!(
            parsed.get("instructions").is_none(),
            "instructions should be removed, got: {result}"
        );
        assert!(
            parsed.get("permission").is_none(),
            "permission should be removed, got: {result}"
        );
        assert!(
            !instruction_path.exists(),
            "instruction file should be removed"
        );

        // Cleanup
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn remove_hook_from_opencode_preserves_unrelated_permissions() {
        let base = std::env::temp_dir().join("vstack_test_opencode_permissions");
        let _ = std::fs::create_dir_all(&base);
        let config_path = base.join("opencode.json");
        let instruction_path = base.join("instructions").join("vstack-hook-review-bash.md");
        std::fs::create_dir_all(instruction_path.parent().unwrap()).unwrap();
        std::fs::write(&instruction_path, "# Safety").unwrap();

        let content = r#"{
  "$schema": "https://opencode.ai/config.json",
  "instructions": [
    "instructions/vstack-hook-review-bash.md"
  ],
  "permission": {
    "edit": "deny",
    "bash": {
      "*": "ask"
    }
  }
}"#;
        std::fs::write(&config_path, content).unwrap();

        remove_hook_from_opencode_json_at_path(
            &config_path,
            &instruction_path,
            "instructions/vstack-hook-review-bash.md",
            "review-bash",
        )
        .unwrap();

        let result = std::fs::read_to_string(&config_path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(
            parsed.get("permission").and_then(|p| p.get("edit")),
            Some(&serde_json::Value::String("deny".into()))
        );
        assert!(
            parsed
                .get("permission")
                .and_then(|p| p.get("bash"))
                .is_none(),
            "vstack-added bash permission should be removed, got: {result}"
        );

        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn relative_path_uses_relative_target_for_normal_directories() {
        let root = std::env::temp_dir().join(format!(
            "vstack_relative_path_normal_{}_{}",
            std::process::id(),
            crate::config::now_iso().replace([':', '-'], "")
        ));
        let from = root.join("a").join("b");
        let to = root.join("config").join("skills").join("rust-async");
        std::fs::create_dir_all(&from).unwrap();
        std::fs::create_dir_all(&to).unwrap();

        let rel = relative_path(&from, &to).unwrap();
        assert_eq!(rel, PathBuf::from("../../config/skills/rust-async"));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[cfg(unix)]
    #[test]
    fn relative_path_uses_absolute_target_when_parent_is_symlinked() {
        use std::os::unix::fs::symlink;

        let root = std::env::temp_dir().join(format!(
            "vstack_relative_path_symlink_{}_{}",
            std::process::id(),
            crate::config::now_iso().replace([':', '-'], "")
        ));
        let real_parent = root.join("real").join("skills");
        let apparent_parent = root.join("apparent");
        let target = root.join("config").join("skills").join("rust-async");

        std::fs::create_dir_all(&real_parent).unwrap();
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&target).unwrap();
        symlink(&real_parent, &apparent_parent).unwrap();

        let rel = relative_path(&apparent_parent, &target).unwrap();
        assert!(
            rel.is_absolute(),
            "expected absolute symlink target, got {rel:?}"
        );
        assert_eq!(rel, std::fs::canonicalize(&target).unwrap());

        let _ = std::fs::remove_dir_all(&root);
    }
}
