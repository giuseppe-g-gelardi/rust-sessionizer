use crate::config::config::{Cfg, CfgManager};
use dialoguer::Select;
use std::error::Error;

// use super::open::open;
// use super::exit::exit;

use super::{exit::exit, open::open};

#[derive(Clone, Debug)]
pub enum Welcome {
    Init,
    Open,
    Update,
    Exit,
}

impl Welcome {
    pub fn execute(&self) -> String {
        match self {
            Welcome::Init => welcome_dialog(),
            Welcome::Open => open(),
            Welcome::Update => update_dialog(),
            Welcome::Exit => exit(),
        }
    }
}

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

pub fn welcome_dialog() -> String {
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
        0 => Welcome::Open.execute(),
        1 => Welcome::Update.execute(),
        2 => Welcome::Exit.execute(),
        _ => Welcome::Exit.execute(),
    }
}

fn update_dialog() -> String {
    // configure_editor?? // i dont think this should return anything
    // NOTE: should return a reference/pointer to a config
    let mut editor = "".to_string();
    let mut alias = "".to_string();
    let mut tmux = "".to_string(); // probably have to do some parsing here

    editor = update_editor();
    alias = update_alias();

    if editor != "vscode" {
        tmux = update_tmux().to_string();
    } else {
        tmux = "false".to_string();
    }

    let config_options = confirm_config_options(&editor, &alias, tmux.parse().unwrap());
    if !config_options {
        update_dialog();
    }

    // get config
    // updated_config = &cfg{editor: editor, alias: alias, tmux: tmux}
    // write_config(updated_config)
    // init_cli()

    "".to_string()
}

fn update_editor() -> String {
    let choices = vec!["vscode", "neovim", "vim", "emacs"];
    let selections = Select::new()
        .items(&choices)
        .default(0)
        .with_prompt("Select an editor")
        .interact()
        .unwrap();

    // println!("selections: {:?}", choices[selections]);
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

fn update_alias() -> String {
    // NOTE: this should be an input field, not a select

    "".to_string()
}

fn update_tmux() -> bool {
    // NOTE: this should be a select, not an input field

    true
}

fn update_config() -> Result<(), Box<dyn Error>> {
    let config_manager = CfgManager::new_cfg_manager();
    let config = &config_manager.get_config(1)?;

    let _ = &config_manager.write_config(&Cfg {
        editor: "neovim".to_string(),
        ..config.clone()
    });

    let updated_config = &config_manager.get_config(1);
    println!("Updated config: {:?}", updated_config);

    Ok(())
}

fn confirm_config_options(editor: &str, alias: &str, tmux: bool) -> bool {
    println!("Your config options are: ");
    println!("Editor: {}", editor);
    if tmux {
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

pub fn update() -> String {
    // let config_manager = CfgManager::new_cfg_manager();
    // let config = &config_manager.get_config(1)?;
    //
    // let _ = &config_manager.write_config(&Cfg {
    //     editor: "neovim".to_string(),
    //     ..config.clone()
    // });
    //
    // let updated_config = &config_manager.get_config(1);
    // println!("Updated config: {:?}", updated_config);
    "update function!!!!".to_string()
}
