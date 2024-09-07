// internal
// use config::config::{Config, ConfigManager};

// standard library
// use std::fs;
use std::io::Error;
// use std::path::Path;

use self::config::config::{Cfg, CfgManager};

// modules
mod config;

// // constants ?
// const FILE_NAME: &str = "cfg.json";

// ************************************************************************** //
// how the control flow should work:
// main fn
// - instante the config manager (ConfigManager::new())
// - - check if the config file exists (config_manager.get_config_file_location())
// - - - if it does, verify the access token (if access_token == "") authenticate()
// - - - if it doesn't, create a new config file (let mut cfg = Config::new())
// - - - - update the config file with default values
// - - - - save the config file
//
// - if the config file exists and the access token is verified ..
// - - start the TUI application
// ************************************************************************** //

fn main() -> Result<(), Error> {
    let config_manager = CfgManager::new_cfg_manager();

    let cfg = Cfg {
        access_token: "banana".to_string(),
        editor: "emacs, ewwwww".to_string(),
        alias: Some("sweet alias".to_string()),
        tmux: true,
    };

    // let no_cfg = &config_manager.write_config(None);
    let with_cfg = &config_manager.write_config(Some(&cfg));
    // println!("no_cfg: {:?}", no_cfg);
    println!("with_cfg: {:?}", with_cfg);

    // let with_cfg = &config_manager.write_config(Some(&cfg));

    // let config_manager = ConfigManager::new();
    // let check_config_exists = &config_manager.get_config(1);

    // match check_config_exists {
    //     Ok(_) => println!("Config file exists"),
    //     Err(e) => eprintln!("Config file does not exist: {:?}", e), // create new config file
    // }
    // println!("Config manager: {:?}", config_manager.config);
    //
    // if config_manager.config.access_token == "" {
    //     println!("Access token is empty, authenticating");// config_manager.authenticate().unwrap();
    // } else {
    //     println!("authenticated");
    // }
    //
    // println!("init tui");
    //
    // let json_cfg = config_manager.config.to_json();
    // println!("json_cfg: {:?}", json_cfg);
    //
    // let file_name: Vec<String> = std::env::args().collect();

    Ok(())
}
