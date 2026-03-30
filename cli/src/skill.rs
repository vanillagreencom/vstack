use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

/// Skill definition parsed from SKILL.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default, rename = "user-invocable")]
    pub user_invocable: Option<bool>,
    /// Structured dependencies from frontmatter
    #[serde(default)]
    pub dependencies: Option<SkillDeps>,
    /// Body markdown
    #[serde(skip)]
    pub body: String,
    /// Directory containing the skill files
    #[serde(skip)]
    pub source_dir: PathBuf,
    /// Resolved dependency list (from frontmatter or body parsing)
    #[serde(skip)]
    pub resolved_deps: Vec<SkillDep>,
}

/// Structured dependency declaration in frontmatter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillDeps {
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default)]
    pub optional: Vec<String>,
}

/// A resolved dependency reference
#[derive(Debug, Clone)]
pub struct SkillDep {
    pub name: String,
    pub optional: bool,
}

impl Skill {
    /// Parse a SKILL.md file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let (frontmatter, body) = crate::frontmatter::split_yaml_frontmatter(&content)?;
        let mut skill: Skill =
            serde_yaml::from_str(&frontmatter).context("parsing skill frontmatter")?;
        skill.source_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        skill.resolved_deps = resolve_dependencies(skill.dependencies.as_ref(), &body);
        skill.body = body;

        Ok(skill)
    }
}

/// Discover all skills in a directory (looks for SKILL.md in subdirs)
pub fn discover_skills(dir: &Path) -> Result<Vec<Skill>> {
    let mut skills = Vec::new();
    if !dir.exists() {
        return Ok(skills);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let skill_file = path.join("SKILL.md");
            if skill_file.exists() {
                match Skill::from_file(&skill_file) {
                    Ok(skill) => skills.push(skill),
                    Err(e) => eprintln!("Warning: skipping {}: {e}", skill_file.display()),
                }
            }
        }
    }
    skills.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(skills)
}

/// Build a dependency graph: skill name → list of required skill names.
pub fn build_dependency_graph(skills: &[Skill]) -> HashMap<String, Vec<String>> {
    let skill_names: HashSet<&str> = skills.iter().map(|s| s.name.as_str()).collect();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for skill in skills {
        let deps: Vec<String> = skill
            .resolved_deps
            .iter()
            .filter(|d| !d.optional)
            .filter(|d| skill_names.contains(d.name.as_str()))
            .map(|d| d.name.clone())
            .collect();
        if !deps.is_empty() {
            graph.insert(skill.name.clone(), deps);
        }
    }

    graph
}

/// Given a set of selected skill names, expand to include all required dependencies.
/// Returns the expanded set and a list of auto-added skills.
pub fn expand_dependencies(
    selected: &[String],
    graph: &HashMap<String, Vec<String>>,
) -> (Vec<String>, Vec<String>) {
    let mut result = selected.to_vec();
    let mut added = Vec::new();
    let mut seen: HashSet<String> = selected.iter().cloned().collect();
    let mut queue: VecDeque<String> = selected.iter().cloned().collect();

    while let Some(name) = queue.pop_front() {
        if let Some(deps) = graph.get(&name) {
            for dep in deps {
                if seen.insert(dep.clone()) {
                    result.push(dep.clone());
                    added.push(dep.clone());
                    queue.push_back(dep.clone());
                }
            }
        }
    }

    (result, added)
}

fn resolve_dependencies(dependencies: Option<&SkillDeps>, body: &str) -> Vec<SkillDep> {
    if let Some(deps) = dependencies {
        let mut resolved = Vec::new();
        for name in &deps.required {
            resolved.push(SkillDep {
                name: name.clone(),
                optional: false,
            });
        }
        for name in &deps.optional {
            resolved.push(SkillDep {
                name: name.clone(),
                optional: true,
            });
        }
        resolved
    } else {
        parse_dependencies_from_body(body)
    }
}

// ── Body fallback parser ─────────────────────────────────────────────

/// Parse dependency table from skill body markdown (fallback when no frontmatter deps).
fn parse_dependencies_from_body(body: &str) -> Vec<SkillDep> {
    let mut deps = Vec::new();
    let mut in_dep_section = false;
    let mut in_table = false;
    let mut is_reverse_dep = false;

    for line in body.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('#') {
            if trimmed.contains("Skill Dependencies") || trimmed.contains("Dependencies") {
                in_dep_section = true;
                in_table = false;
                is_reverse_dep = false;
                continue;
            } else if in_dep_section {
                break;
            }
        }

        if !in_dep_section {
            continue;
        }

        if trimmed.contains("self-contained")
            || trimmed.contains("depend on it")
            || trimmed.contains("Dependent Skill")
        {
            is_reverse_dep = true;
        }

        if trimmed.starts_with("Project-level") || trimmed.starts_with("**Project") {
            break;
        }

        if is_reverse_dep {
            continue;
        }

        if trimmed.starts_with('|') && trimmed.ends_with('|') {
            if trimmed.contains("---") {
                in_table = true;
                continue;
            }
            if !in_table {
                if trimmed.contains("Dependent") {
                    is_reverse_dep = true;
                }
                continue;
            }

            let optional = trimmed.contains("(optional)");

            let cols: Vec<&str> = trimmed.split('|').collect();
            if cols.len() >= 2 {
                let dep_col = cols[1].trim();
                let mut found = false;

                // Strategy 1: backtick-quoted names
                let parts: Vec<&str> = dep_col.split('`').collect();
                let mut i = 1;
                while i < parts.len() {
                    let candidate = parts[i].trim();
                    if !candidate.is_empty()
                        && !candidate.contains(' ')
                        && !candidate.starts_with('$')
                        && candidate
                            .chars()
                            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
                    {
                        deps.push(SkillDep {
                            name: candidate.to_string(),
                            optional,
                        });
                        found = true;
                    }
                    i += 2;
                }

                // Strategy 2: "Xxx Yyy skill" pattern
                if !found {
                    let clean = dep_col
                        .replace("(optional)", "")
                        .replace("(e.g.,", "")
                        .trim()
                        .to_string();
                    if let Some(name_part) = clean.strip_suffix("skill") {
                        let name_part = name_part.trim();
                        if !name_part.is_empty() {
                            let kebab: String = name_part
                                .split_whitespace()
                                .map(|w| w.to_lowercase())
                                .collect::<Vec<_>>()
                                .join("-");
                            deps.push(SkillDep {
                                name: kebab,
                                optional,
                            });
                        }
                    }
                }
            }
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_frontmatter_deps() {
        let content = r#"---
name: test-skill
description: Test
dependencies:
  required: [linear, decider]
  optional: [benchmarking]
---

# Test
"#;
        let skill = Skill::from_file(Path::new("/dev/null")).unwrap_or_else(|_| {
            // Parse directly since /dev/null won't work
            let (fm, body) = crate::frontmatter::split_yaml_frontmatter(content).unwrap();
            let mut s: Skill = serde_yaml::from_str(&fm).unwrap();
            s.body = body;
            s.resolved_deps = resolve_dependencies(s.dependencies.as_ref(), &s.body);
            s
        });

        assert_eq!(skill.resolved_deps.len(), 3);
        assert!(
            skill
                .resolved_deps
                .iter()
                .any(|d| d.name == "linear" && !d.optional)
        );
        assert!(
            skill
                .resolved_deps
                .iter()
                .any(|d| d.name == "decider" && !d.optional)
        );
        assert!(
            skill
                .resolved_deps
                .iter()
                .any(|d| d.name == "benchmarking" && d.optional)
        );
    }

    #[test]
    fn parse_body_fallback_deps() {
        let body = r#"
# My Skill

## Skill Dependencies

| Dependency | Purpose | Variable |
|------------|---------|----------|
| Issue tracker CLI (e.g., `linear` skill) | Issue CRUD | `$ISSUE_CLI` |
| Orchestration skill | Review-finding schema | Referenced by name |
| Benchmarking skill (optional) | Baseline capture | `$BENCH_CLI` |

## Other Section
"#;
        let deps = parse_dependencies_from_body(body);
        let names: Vec<&str> = deps.iter().map(|d| d.name.as_str()).collect();
        assert!(names.contains(&"linear"));
        assert!(names.contains(&"orchestration"));
        assert!(names.contains(&"benchmarking"));
        assert!(
            deps.iter()
                .find(|d| d.name == "benchmarking")
                .unwrap()
                .optional
        );
    }

    #[test]
    fn expand_deps_transitive() {
        let mut graph = HashMap::new();
        graph.insert(
            "orchestration".into(),
            vec!["issue-lifecycle".into(), "decider".into()],
        );
        graph.insert("issue-lifecycle".into(), vec!["linear".into()]);

        let selected = vec!["orchestration".to_string()];
        let (expanded, added) = expand_dependencies(&selected, &graph);

        assert!(expanded.contains(&"issue-lifecycle".to_string()));
        assert!(expanded.contains(&"decider".to_string()));
        assert!(expanded.contains(&"linear".to_string()));
        assert_eq!(added.len(), 3);
    }
}
