//! The commit message kendex offers.
//!
//! The rule is the command, not a list, so no enumeration can fall out of
//! date as the command surface changes.

/// `chore: kendex <command>`, where `<command>` is what the person typed
/// without its flags and arguments.
///
/// A subcommand is named with its group, because the group alone is not a
/// command anybody can run: `chore: kendex marketplace subscribe`. A verb
/// that delegates to another names itself, not the one it called —
/// `kendex updates` runs `refresh`'s plan and its message says `updates`,
/// because that is what the person ran.
///
/// The message has no body.
pub fn default_message(command: &str) -> String {
    format!("chore: kendex {}", command.trim())
}
