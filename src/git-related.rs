///
/// # git_related
/// Contains functions related to git.
// Imports ================================================================================= Imports
use ansi_term::Colour::{Green, Red};
use std::process::Command;

// Functions  ===========================================================================  Functions

// GIT FUNCTIONS ===================================================================== GIT FUNCTIONS

///
/// # commit
/// Commits the changes.
///
/// ## Arguments
/// * `message` - `String` - The commit message
/// * `verbose` - `bool` - If the commit should be verbose or not
///
/// ## Returns
/// * `Result<(), String>` - The result of the commit
pub fn commit(message: String, verbose: bool) -> Result<bool, String> {
    if verbose {
        println!("Commiting...");
    }

    let final_args: Vec<&str> = vec!["commit", "-m", message.as_str()];

    let command = Command::new("git")
        .args(final_args)
        .output()
        .expect("failed to execute process");

    if command.status.success() {
        println!("{}", Green.bold().paint("Commit successful."));

        Ok(true)
    } else {
        println!("{}", Red.bold().paint("Commit failed."));

        Err("Commit failed.".to_string())
    }
}

// Tests ==================================================================================== Tests
