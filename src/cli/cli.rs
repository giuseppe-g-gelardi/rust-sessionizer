use crate::{
    config::config::{Cfg, CfgManager},
    repo::repo::PartialRepo,
};
use dialoguer::{Input, Select};

use super::{exit::exit, open::open};

pub enum Editor {
    Vscode,
    Neovim,
    Vim,
    Emacs,
}

impl Editor {
    pub fn set(&self) -> String {
        match self {
            Editor::Vscode => "vscode".to_string(),
            Editor::Neovim => "neovim".to_string(),
            Editor::Vim => "vim".to_string(),
            Editor::Emacs => "emacs".to_string(),
        }
    }
}

pub fn init(cm: &CfgManager, repos: Vec<PartialRepo>) {
    print!("\x1B[2J\x1B[1;1H");
    let choices = vec!["Open", "Update", "Exit"];

    let selections = Select::new()
        .items(&choices)
        .default(0)
        .with_prompt(
            "\nWould you like to:\n- Open a repo?\n- Update your config?\n- Exit the program?\n",
        )
        .interact()
        .unwrap();

    match selections {
        0 => open(cm, repos),
        1 => update_config(cm, repos),
        2 => exit(),
        _ => exit(),
    };
}

fn update_config(cm: &CfgManager, repos: Vec<PartialRepo>) {
    let editor = update_editor();
    let alias = update_alias();

    let tmux = if editor != "vscode" {
        update_tmux()
    } else {
        false
    };

    let _ = confirm_config_options(&editor, &alias, &tmux);
    // if !config_options {
    //     update_config(cm);
    // }

    let _ = &cm.write_config(&Cfg {
        editor,
        alias: Some(alias),
        tmux,
        ..cm.get_config(1).unwrap()
    });

    init(cm, repos);
}

fn update_editor() -> String {
    let choices = vec!["vscode", "neovim", "vim", "emacs"];
    let selections = Select::new()
        .items(&choices)
        .default(0)
        .with_prompt("Select an editor")
        .interact()
        .unwrap();

    print!("selections: {:?}", choices[selections]);

    match selections {
        0 => Editor::Vscode.set(),
        1 => Editor::Neovim.set(),
        2 => Editor::Vim.set(),
        3 => Editor::Emacs.set(),
        _ => Editor::Vscode.set(), // vscode is the default option
    };

    choices[selections].to_string()
}

fn is_using_alias() -> bool {
    let is_using_alias = Select::new()
        .items(&["Yes", "No"])
        .default(0)
        .with_prompt("Would you like to set an alias?")
        .interact()
        .unwrap();

    match is_using_alias {
        0 => true,
        1 => false,
        _ => false,
    }
}

fn update_alias() -> String {
    if let true = is_using_alias() {
        let alias = Input::<String>::new()
            .with_prompt("Enter an alias")
            .interact()
            .unwrap();

        alias
    } else {
        "".to_string()
    }
}

fn update_tmux() -> bool {
    let choices = vec!["Yes", "No"];
    let selections = Select::new()
        .items(&choices)
        .default(0)
        .with_prompt("Would you like to use tmux?")
        .interact()
        .unwrap();

    match selections {
        0 => true,
        1 => false,
        _ => false,
    }
}

fn confirm_config_options(editor: &str, alias: &str, tmux: &bool) -> bool {
    println!("Your config options are: ");
    println!("Editor: {}", editor);
    if *tmux {
        println!("tmux: {}", tmux);
    }
    if alias != "" {
        // !alias.is_empty() ??
        println!("Alias: {}", alias);
    }

    let choices = vec!["Yes!", "Nope!"];
    let selections = Select::new()
        .items(&choices)
        .default(0)
        .with_prompt("YES: I am happy with these settings. -- NO: I want to change them!")
        .interact()
        .unwrap();

    match selections {
        0 => true,
        1 => false,
        _ => false,
    }
}
