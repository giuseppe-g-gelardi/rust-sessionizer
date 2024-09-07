use serde::{Deserialize, Serialize};
use serde_json::json;

// use std::fmt::write;
use std::fs;
use std::io::Error;

use dirs;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Cfg {
    pub access_token: String,
    pub editor: String,
    pub alias: Option<String>,
    pub tmux: bool,
}

#[derive(Debug)]
pub struct CfgManager {
    // config_file_name: Option<PathBuf>,
    pub config_file_name: String,
    pub config: Cfg,
}

impl CfgManager {
    pub fn new_cfg_manager() -> Self {
        // let config_file_name = self.get_config_file_location();
        let config_file_name = "cfg.json".to_string();
        let default_config: Cfg = Cfg {
            access_token: "".to_string(),
            editor: "vscode".to_string(),
            alias: None,
            tmux: false,
        };

        Self {
            config_file_name,
            config: default_config,
        }
    }

    fn _get_config_file_location(self) -> Option<PathBuf> {
        let home_dir = dirs::home_dir()?; // Get user's home directory
        let config_dir = match std::env::consts::OS {
            "windows" => {
                let appdata = env::var("APPDATA").ok()?;
                PathBuf::from(appdata).join("local")
            }
            "linux" | "macos" => home_dir.join(".config"),
            _ => {
                eprintln!("Unsupported operating system");
                return None;
            }
        };

        Some(config_dir.join("cfg.json"))
    }

    pub fn get_config(&self, recursive_depth: u8) -> Result<Cfg, Error> {
        if recursive_depth > 5 {
            return Err(Error::new(std::io::ErrorKind::NotFound, "File not found"));
        }

        // match fs::read_to_string(self.config_file_name.as_ref().unwrap()) {
        match fs::read_to_string(self.config_file_name.to_string()) {
            Ok(content) => match serde_json::from_str::<Cfg>(&content) {
                Ok(config) => Ok(config),
                Err(e) => Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid data, {}".to_string() + &e.to_string(),
                )),
            },
            Err(_) => {
                // self.create_default_config_file("cfg.json".to_string());
                self.get_config(recursive_depth + 1)
            }
        }
    }

    pub fn write_config(self, config: Option<&Cfg>) -> Result<Cfg, Error> {
        // might need to take a page from go's playbook and do some error handling
        let mut result = self.save_to_file(None); // ??????

        // needs to be updated slightly
        //
        // IF the config file does NOT exist
        // save default config to file
        // - create the default config file
        //
        // ELSE IF the config file DOES exist
        // - AND IF the config is None (which means its been set in some way shape or form)
        // - RETURN - DONT DO ANYTHING - keep it movin!
        //
        // ELSE
        // - update the config file with the new values
        // - save the updated config file 
        //
        // probalby should use a match statement here
        if config.is_none() {
            println!("config is None");
        } else {
            println!("config is not None: {:#?}", json!(&config));
            result = self.save_to_file(config); // update the config with the new values
        }

        println!("result: {:#?}", result);
        return self.get_config(0);
    }

    pub fn save_to_file(&self, config: Option<&Cfg>) -> std::io::Result<()> {
        // there has to be a better way to do this
        if config.is_none() {
            let json_cfg = json!(self.config);
            let json_str = serde_json::to_string_pretty(&json_cfg).unwrap();
            std::fs::write(&self.config_file_name, json_str)
        } else {
            let json_cfg = json!(&config);
            let json_str = serde_json::to_string_pretty(&json_cfg).unwrap();
            std::fs::write(&self.config_file_name, json_str)
        }
    }

    // pub fn save_to_file(&self, file_name: &str) -> std::io::Result<()> {
    //     let json_cfg = self.to_json();
    //     let json_str = serde_json::to_string_pretty(&json_cfg).unwrap();
    //     std::fs::write(file_name, json_str)
    // }
}

// ************************************************************************** //
// ************************************************************************** //
// ************************************************************************** //
// ************************************************************************** //
// ************************************************************************** //
//
//
//
//
// #[derive(Debug, Deserialize)]
// pub struct Config {
//     // should specify if this is json?
//     pub access_token: String,
//     pub editor: String,        // vscode, neovim, vim, emacs, etc.
//     pub alias: Option<String>, // user defined alias
//     pub tmux: bool,
// }
//
// #[derive(Debug)]
// pub struct ConfigManager {
//     pub config_file_name: String,
//     pub config: Config,
// }
//
// impl ConfigManager {
//     pub fn new() -> Self {
//         let config_file_name = "cfg.json".to_string(); //get_config_file_location()
//         let mut config = Config::new();
//
//         config.access_token = "".to_string();
//         config.editor = "vscode".to_string();
//         config.alias = None;
//         config.tmux = false;
//
//         Self {
//             config_file_name,
//             config,
//         }
//     }
//
//     pub fn _get_config_file_location(&self) -> Option<PathBuf> {
//         let home_dir = dirs::home_dir()?; // Get user's home directory
//
//         /*
//         - for linux and mac it is ~/.config/cfg.json -- subject to change
//         - for windows it is %APPDATA%\local\cfg.json ???
//         */
//
//         // Determine config directory based on operating system
//         let config_dir = match std::env::consts::OS {
//             "windows" => {
//                 // Get APPDATA environment variable for Windows
//                 let appdata = env::var("APPDATA").ok()?;
//                 PathBuf::from(appdata).join("local")
//             }
//             "linux" | "macos" => {
//                 // Use user's home directory and append .config for Linux and macOS
//                 home_dir.join(".config")
//             }
//             _ => {
//                 eprintln!("Unsupported operating system");
//                 return None;
//             }
//         };
//
//         // Create "cfg.json" path
//         Some(config_dir.join("cfg.json"))
//     }
//
//     pub fn get_config(&self, recursive_depth: u8) -> Result<Config, Error> {
//         if recursive_depth > 5 {
//             return Err(Error::new(std::io::ErrorKind::NotFound, "File not found"));
//         }
//
//         match fs::read_to_string(&self.config_file_name) {
//             Ok(content) => match serde_json::from_str::<Config>(&content) {
//                 Ok(config) => Ok(config),
//                 Err(e) => Err(Error::new(
//                     std::io::ErrorKind::InvalidData,
//                     "Invalid data, {}".to_string() + &e.to_string(),
//                 )),
//             },
//             Err(_) => {
//                 // Error reading config file, create a new default config file
//                 self.create_default_config_file("cfg.json".to_string());
//                 self.get_config(recursive_depth + 1)
//             }
//         }
//     }
//
//     // pub fn write_config(&self, ????) {
//     // // if params is None, write the default config
//     // // else update the corresponding fields
//     // }
//
//     pub fn create_default_config_file(&self, file_name: String) {
//         let mut cfg = Config::new();
//
//         cfg.access_token = "".to_string();
//         cfg.editor = "vscode".to_string();
//         cfg.alias = None;
//         cfg.tmux = false;
//
//         cfg.save_to_file(&file_name).unwrap();
//     }
// }
//
// impl Config {
//     pub fn new() -> Self {
//         Self {
//             access_token: "".to_string(),
//             editor: "".to_string(),
//             alias: None,
//             tmux: false,
//         }
//     }
//
//     pub fn to_json(&self) -> serde_json::Value {
//         json!({
//             "access_token": self.access_token,
//             "editor": self.editor,
//             "alias": self.alias,
//             "tmux": self.tmux,
//         })
//     }
//
//     pub fn save_to_file(&self, file_name: &str) -> std::io::Result<()> {
//         let json_cfg = self.to_json();
//         let json_str = serde_json::to_string_pretty(&json_cfg).unwrap();
//         std::fs::write(file_name, json_str)
//     }
//
//     pub fn create_config(&self, file_name: &String) -> Result<(), Error> {
//         self.save_to_file(file_name)?;
//         Ok(())
//     }
//
//     pub fn update_config_file(&mut self, file_name: &String) {
//         // let mut cfg = Config::new();
//
//         self.access_token = "asdfasdf".to_string();
//         self.editor = "vscode".to_string();
//         self.alias = None;
//         self.tmux = false;
//
//         self.save_to_file(&file_name).unwrap();
//     }
//
//     // for some reason this is reading the whole config...
//     // really need to narrow down whats happening because
//     // it seems like i have 5 functions doing effectively the same thing
//     pub fn verify_access_token(&self, file_name: &String) -> Result<(), Error> {
//         // this next line is pissing me off. I don't know what to do with it
//         // keeps crashing the program instead of returning an error and moving
//         // on with the rest of the program to circumvent the absence of the file
//         // it is looking for.
//         let text = fs::read_to_string(file_name);
//         println!("!!!!!!!!!text: {:#?}", &text);
//
//         // not sure what to do in here
//
//         // let cfg: Config = from_str(&text)?;
//         // println!("access_token: {}", self.access_token);
//
//         if self.access_token != "" {
//             println!("access_token: {}", self.access_token);
//         } else {
//             println!("no access token")
//         }
//
//         Ok(())
//     }
// }
