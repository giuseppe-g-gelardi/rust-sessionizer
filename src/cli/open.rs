use crate::config::config::{Cfg, CfgManager};
use crate::repo::repo::PartialRepo;

use dialoguer::Select;

pub fn open(cm: &CfgManager, repos: Vec<PartialRepo>) {
    let selected_repo = repo_selection(repos);
    let config = cm.get_config(1).unwrap();
    let editor_command = set_editor_command(&config);
    let repo_url = set_repo_url(&selected_repo);
    let bare_repo = is_bare_repo();
    let cmd = command_builder(&repo_url, bare_repo);

    println!("editor_command: {:#?}", editor_command);
    println!("repo_url: {:#?}", repo_url);
    println!("bare_repo: {:#?}", bare_repo);
    println!("cmd: {:#?}", cmd);

    /*
     *
     * cd into the directory
     * command builder and command runner
     * check if tmux is active, promptfor:
     * - attach to existing session /new window
     * - create new session
     *
     * open editor (with cloned repo)
     *
     */
}

fn repo_selection(repos: Vec<PartialRepo>) -> PartialRepo {
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

    repos[selections].clone()
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
    let choices = vec!["html", "ssh"];
    let selections = Select::new()
        .items(&choices)
        .default(0)
        .with_prompt("Would you like to clone via html or ssh?")
        .interact()
        .unwrap();

    let repo_url = if selections == 0 {
        repo.html_url.to_string()
    } else {
        repo.ssh_url.to_string()
    };

    repo_url
}

fn is_bare_repo() -> bool {
    let choices = vec!["regular (recommended)", "bare"];
    let selections = Select::new()
        .items(&choices)
        .default(0)
        .with_prompt("Would you like to clone as a bare repo? not recommended unless you know what you're doing")
        .interact()
        .unwrap();

    let is_bare = if selections == 1 { true } else { false };

    is_bare
}
