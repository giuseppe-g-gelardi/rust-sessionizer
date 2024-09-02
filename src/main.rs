// internal
use config::config::{Config, ConfigManager};

// standard library
use std::fs;
use std::io::Error;
use std::path::Path;

// modules
mod config;

// constants ?
const FILE_NAME: &str = "cfg.json";

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
    let config_manager = ConfigManager::new();

    let check_config_exists = &config_manager.get_config(1);
    match check_config_exists {
        Ok(_) => println!("Config file exists"),
        Err(e) => eprintln!("Config file does not exist: {:?}", e), // create new config file
    }
    println!("Config manager: {:?}", config_manager.config);

    if config_manager.config.access_token == "" {
        println!("Access token is empty, authenticating");
        // config_manager.authenticate().unwrap();
    } else {
        println!("authenticated");
    }

    println!("init tui");

    let json_cfg = config_manager.config.to_json();
    println!("json_cfg: {:?}", json_cfg);

    // let file_location = config_manager.get_config_file_location();
    // match file_location {
    //     //
    //     Some(path) => println!("Config file location: {:?}", path),
    //     None => eprintln!("Failed to get config file location"),
    // }

    // let text = fs::read_to_string(file_name.to_string())?;
    let file_name: Vec<String> = std::env::args().collect();

    //
    //
    //
    // let file_exists = verify_config_exists(&file_name[1]);

    // this should probably happen in the ConfigManager
    // let mut cfg = Config::new();

    // cfg.update_config_file(&FILE_NAME.to_string()); // update_config_file();

    // verify_file_exists(&cfg, file_name, file_exists)?; // verify_file_exists(&cfg, file_name,

    Ok(())
}

// fn verify_file_exists(
//     cfg: &Config,
//     file_name: Vec<String>,
//     file_exists: Result<(), Error>,
// ) -> Result<(), Error> {
//     match file_exists {
//         Ok(_) => {
//             println!("file existsIN VERIFY_FILE_EXISTS!!!!!!");
//             cfg.verify_access_token(&file_name[1]).unwrap();
//         }
//         Err(e) => {
//             println!(
//                 "file does not exist, creating new config file. {:?}",
//                 e.to_string()
//             );
//             cfg.create_config(&FILE_NAME.to_string()).unwrap();
//         }
//     }
//
//     Ok(())
// }
//
// fn verify_config_exists(file_name: &String) -> Result<(), Error> {
//     if Path::new(file_name).exists() {
//         let x = fs::read_to_string(file_name)?; // let text = ????? // need to get rid of this ? lol
//         println!("xxxxxxx: {:?}", x);
//         Ok(())
//     } else {
//         Err(Error::new(std::io::ErrorKind::NotFound, "File not found"))
//     }
// }
