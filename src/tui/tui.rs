use crate::config::config::{Cfg, CfgManager};
use dialoguer::Select;
use std::error::Error;

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

pub fn open() -> String {
    // TODO:
    // octocrab?? use github api, get list of users repos, append public|private to each
    // put in list
    // logic to open repo, editor, tmux, etc....
    "open function!!!!".to_string()
}

//
fn update_dialog() -> String {
    let choices = vec!["vscode", "neovim", "vim", "emacs"];
    let selections = Select::new()
        .items(&choices)
        .default(0)
        .with_prompt("Select an editor")
        .interact()
        .unwrap();

    println!("selections: {:?}", choices[selections]);

    // TODO: call the update_config() function, and pass in choices[selections]

    // "update function!!!!".to_string();
    choices[selections].to_string()
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
//

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

pub fn exit() -> String {
    // NOTE: just like, exit the program...
    "exit function!!!!".to_string()
}
