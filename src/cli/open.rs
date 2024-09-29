use crate::cli::cli::init;
use crate::config::config::CfgManager;
use crate::repo::repo::PartialRepo;

use dialoguer::Select;

pub fn open(cm: &CfgManager, repos: Vec<PartialRepo>) {
    let repo_select_options = repos
        .iter()
        .map(|r| format!("{} - {} - {}", r.name, r.visibility, r.description))
        .collect::<Vec<String>>();

    let selections = Select::new()
        .items(&repo_select_options)
        .default(0)
        .with_prompt("Select a repository to open")
        .interact()
        .unwrap();

    let selected_repo = &repos[selections].clone(); // -> gets passed to the next function
    println!("Opening repo: {:#?}", selected_repo);

    init(cm, repos)
}
