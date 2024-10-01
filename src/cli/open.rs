use crate::config::config::{Cfg, CfgManager};
use crate::repo::repo::PartialRepo;

use dialoguer::Select;
use std::error::Error;

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

    println!("Selected repo: {:#?}", selected_repo);
    println!("user config: {:#?}", cm.get_config(1).unwrap());

    let config = cm.get_config(1).unwrap();
    let editor_command = set_editor_command(&config);
    let repo_url = set_repo_url(selected_repo);
    let bare_repo = is_bare_repo();
}

fn command_builder(repo_url: &str, is_bare: bool) -> Vec<&str> {
    let cmd = if is_bare {
        vec!["git", "clone", "--bare", repo_url]
    } else {
        vec!["git", "clone", repo_url]
    };
    cmd
}

fn set_editor_command(config: &Cfg) -> String {
    /*
     * if alias is set, for example "c" for vscode, return alias
     * else return editor. for example "neovim" // so the command to open will be "neovim ."
     */
    if config.alias.is_some() && config.alias.as_ref().unwrap() != "" {
        return config.alias.as_ref().unwrap().to_string();
    }
    config.editor.to_string()
}

fn set_repo_url(repo: &PartialRepo) -> String {
    // result of user prompt to clone via "html" or "ssh"
    let repo_url = ""; // html_ssh() prompt

    if repo_url == "ssh" {
        return repo.ssh_url.to_string();
    }
    repo.html_url.to_string()
}

fn is_bare_repo() -> bool {
    // result of user prompt to clone via "bare" or "html"
    let bare_repo = ""; // bare_html() prompt

    if bare_repo == "bare" {
        return true;
    }
    false
}
