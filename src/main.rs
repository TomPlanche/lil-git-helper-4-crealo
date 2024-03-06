/// # git-utils-crealo
/// This file contains the source code for the git-utils-crealo project.
/// Its goal is to provide a set of utilities to work with git repositories.

#[path = "git-related.rs"]
mod git_related;

#[path = "my_theme.rs"]
mod my_theme;

use dialoguer::{Confirm, Select};

const COMMIT_TYPES: [&str; 4] = ["chore", "feat", "fix", "test"];

fn main() {
    let chosen_branch = &COMMIT_TYPES[Select::with_theme(&my_theme::ColorfulTheme::default())
        .default(0)
        .items(&COMMIT_TYPES)
        .interact()
        .unwrap()];

    // scope could be empty
    let scope = dialoguer::Input::<String>::new()
        .allow_empty(true)
        .with_prompt("Enter scope")
        .interact()
        .unwrap();

    let commit_message = dialoguer::Input::<String>::new()
        .with_prompt("Enter commit message")
        .interact()
        .unwrap();

    let commit_message = if scope.is_empty() {
        format!("{}: {}", chosen_branch, commit_message)
    } else {
        format!("{}({}): {}", chosen_branch, scope, commit_message)
    };

    if Confirm::new()
        .with_prompt(&format!(
            "Do you want to commit with the message: {}",
            commit_message
        ))
        .interact()
        .unwrap()
    {
        git_related::commit(commit_message, true).unwrap();
    }
}
