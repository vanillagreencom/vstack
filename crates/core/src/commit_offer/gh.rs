//! The one `gh` question the offer asks before it draws, and the one it
//! asks after a push.

use crate::process::Hardened;

use super::{Failed, Refusal, Step, Unavailable, git};

/// A pull request already open for a branch.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct OpenPullRequest {
    pub number: u64,
    pub url: String,
}

#[derive(serde::Deserialize)]
struct Row {
    number: u64,
    url: String,
}

/// Whether the pull-request choice stands, and whether one is already open
/// for this branch.
///
/// One command answers three questions at once — `gh` on the machine, a
/// credential it will use, and a remote it recognises as a GitHub
/// repository — and a fourth the offer asks anyway. kendex runs no
/// separate credential check: `gh auth token` would print the token on a
/// pipe kendex captures, and a probe that does real work answers the same
/// question without one.
///
/// `--repo` binds it to the remote the offer chose. Without it `gh`
/// resolves the repository from the remotes itself, so a project whose
/// `origin` is one host and whose second remote is GitHub would be probed
/// against a repository the push never reaches.
pub fn probe(repo: &str, branch: &str) -> Result<Option<OpenPullRequest>, Unavailable> {
    let output = git::run(
        Hardened::gh(&[
            "pr",
            "list",
            "--repo",
            repo,
            "--head",
            branch,
            "--state",
            "open",
            "--json",
            "number,url",
        ]),
        Step::Probe,
    );
    let rows = match output {
        Ok(stdout) => stdout,
        Err(failed) => return Err(why(&failed)),
    };
    // A `gh` that exits zero and answers with something this cannot read is
    // still a `gh` that works and a repository it recognises, so the choice
    // stands; what it could not tell us is whether one is already open, and
    // the create below answers that with its own refusal.
    Ok(serde_json::from_slice::<Vec<Row>>(&rows)
        .ok()
        .and_then(|rows| rows.into_iter().next())
        .map(|row| OpenPullRequest {
            number: row.number,
            url: row.url,
        }))
}

/// Why the pull-request choice is not on offer.
///
/// A failure to spawn is `gh` not being installed. Everything else is
/// `gh`'s own first line — not signed in, no remote it recognises as a
/// GitHub host, or a case nobody anticipated, which still names itself
/// rather than reading as one of the two above.
pub(super) fn why(failed: &Failed) -> Unavailable {
    match &failed.refusal {
        Refusal::NotStarted(_) => Unavailable::GhMissing,
        Refusal::TimedOut => Unavailable::GhSaid(format!(
            "{} did not finish within {} seconds",
            failed.step.name(),
            failed.step.seconds()
        )),
        // A `gh` that ran and refused always says something; an empty
        // stderr and stdout from a non-zero exit is a `gh` nobody can act
        // on, and reading it as "not installed" would be a claim about a
        // program that is plainly there.
        Refusal::Said(lines) => Unavailable::GhSaid(match lines.first() {
            Some(first) => first.clone(),
            None => "gh exited without saying why".to_owned(),
        }),
    }
}
