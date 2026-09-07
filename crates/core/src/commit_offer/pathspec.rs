//! How the set reaches git.
//!
//! The set runs to a thousand paths in a large project, tens of kilobytes
//! of path text. Windows caps a command line at 32,767 characters and
//! kendex supports Windows, so no step passes the set as arguments: it is
//! written to a file in the system temp directory, NUL-separated, and
//! passed with `--pathspec-from-file` and `--pathspec-file-nul`. `git add`,
//! `git commit` and `git reset` all take it.
//!
//! The file is outside the checkout, so it is never a path the offer could
//! then find, and it is removed when the step ends.

use std::io::Write;
use std::path::PathBuf;

use super::{Failed, Refusal, Step};

/// A NUL-separated pathspec file, removed when it goes out of scope.
pub struct Spec {
    path: PathBuf,
}

impl Spec {
    /// Write the paths for one step.
    pub fn write(paths: &[String], step: Step) -> Result<Spec, Failed> {
        let mut file = tempfile::Builder::new()
            .prefix("kendex-paths-")
            .tempfile()
            .map_err(|error| failure(step, &error))?;
        for path in paths {
            file.write_all(path.as_bytes())
                .and_then(|()| file.write_all(&[0]))
                .map_err(|error| failure(step, &error))?;
        }
        file.flush().map_err(|error| failure(step, &error))?;
        // Kept by path rather than by handle: git opens the file itself,
        // and Windows will not open a second handle to a file this process
        // still holds exclusively.
        let (_, path) = file.keep().map_err(|error| failure(step, &error.error))?;
        Ok(Spec { path })
    }

    /// The two arguments git reads the file through.
    pub fn args(&self) -> [String; 2] {
        [
            format!("--pathspec-from-file={}", self.path.display()),
            "--pathspec-file-nul".to_owned(),
        ]
    }

    /// The git-wide option every step passing this file carries, placed
    /// before the subcommand, where git reads it.
    ///
    /// `--pathspec-file-nul` fixes the separator, not the matching: git
    /// still reads each entry as a pathspec, so a rendered path holding
    /// `[`, `*` or `?` would match a different file and put a path in the
    /// commit that was never in the set. This takes every entry as the
    /// path it is, including one beginning with the `:` a pathspec magic
    /// prefix starts with.
    pub const LITERAL: &'static str = "--literal-pathspecs";
}

impl Drop for Spec {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// A pathspec file that could not be written is that step failing before
/// it ran: nothing was staged, nothing was committed.
fn failure(step: Step, error: &std::io::Error) -> Failed {
    Failed {
        step,
        refusal: Refusal::Said(vec![format!(
            "the list of paths for this step could not be written: {error}"
        )]),
    }
}
