// use crate::cli::cli::init;
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
    // println!("Opening repo: {:#?}", selected_repo);

    // init(cm, repos)
    // NOTE: this shouldnt go back to init, 
    //
    // the next step is to have prompts for: 
    // - bare repo, 
    // - html/ssh, 
    // - and start the session
    next_step(cm, selected_repo);
}

fn next_step(cm: &CfgManager, selected_repo: &PartialRepo) {
    println!("Selected repo: {:#?}", selected_repo);
    println!("user config: {:#?}", cm.get_config(1).unwrap());
    // let choices = vec!["Bare Repo", "HTML/SSH"];
    // let selections = Select::new()
    //     .items(&choices)
    //     .default(0)
    //     .with_prompt("Select a repository to open")
    //     .interact()
    //     .unwrap();
    //
    // match selections {
    //     0 => open_bare_repo(cm, selected_repo),
    //     1 => open_html_ssh(cm, selected_repo),
    //     _ => exit(),
    // };

}
