use serde::Deserialize;
use serde_json::json;

use std::fs;
use std::io::Error;

use dirs;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    // should specify if this is json?
    pub access_token: String,
    pub editor: String,        // vscode, neovim, vim, emacs, etc.
    pub alias: Option<String>, // user defined alias
    pub tmux: bool,
}

#[derive(Debug)]
pub struct ConfigManager {
    pub config_file_name: String,
    pub default_config: Config,
}

impl ConfigManager {
    pub fn new() -> Self {
        let config_file_name = "cfg.json".to_string(); // getConfigFileLocation()
        let mut default_config = Config::new();

        default_config.access_token = "".to_string();
        default_config.editor = "vscode".to_string();
        default_config.alias = None;
        default_config.tmux = false;

        Self {
            config_file_name,
            default_config,
        }
    }

    pub fn get_config_file_location(&self) -> Option<PathBuf> {
        let home_dir = dirs::home_dir()?; // Get user's home directory

        // Determine config directory based on operating system
        let config_dir = match std::env::consts::OS {
            "windows" => {
                // Get APPDATA environment variable for Windows
                let appdata = env::var("APPDATA").ok()?;
                PathBuf::from(appdata).join("local")
            }
            "linux" | "macos" => {
                // Use user's home directory and append .config for Linux and macOS
                home_dir.join(".config")
            }
            _ => {
                eprintln!("Unsupported operating system");
                return None;
            }
        };

        // Create "cfg.json" path
        Some(config_dir.join("cfg.json"))
    }

    pub fn get_config(&self, recursive_depth: u8) -> Result<Config, Error> {
        if recursive_depth > 5 {
            return Err(Error::new(std::io::ErrorKind::NotFound, "File not found"));
        }

        match fs::read_to_string(&self.config_file_name) {
            Ok(content) => match serde_json::from_str::<Config>(&content) {
                Ok(config) => Ok(config),
                Err(e) => Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid data, {}".to_string() + &e.to_string(),
                )),
            },
            Err(_) => {
                // Error reading config file, making a new one // self.write_config(None)?;
                self.get_config(recursive_depth + 1)
            }
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            access_token: "".to_string(),
            editor: "".to_string(),
            alias: None,
            tmux: false,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "access_token": self.access_token,
            "editor": self.editor,
            "alias": self.alias,
            "tmux": self.tmux,
        })
    }

    pub fn save_to_file(&self, file_name: &str) -> std::io::Result<()> {
        let json_cfg = self.to_json();
        let json_str = serde_json::to_string_pretty(&json_cfg).unwrap();
        std::fs::write(file_name, json_str)
    }

    pub fn create_config(&self, file_name: &String) -> Result<(), Error> {
        // let cfg = Config::new();
        self.save_to_file(file_name)?;
        Ok(())
    }

    pub fn update_config_file(&mut self, file_name: &String) {
        // let mut cfg = Config::new();

        self.access_token = "asdfasdf".to_string();
        self.editor = "vscode".to_string();
        self.alias = None;
        self.tmux = false;

        self.save_to_file(&file_name).unwrap();
    }

    // for some reason this is reading the whole config...
    // really need to narrow down whats happening because
    // it seems like i have 5 functions doing effectively the same thing
    pub fn verify_access_token(&self, file_name: &String) -> Result<(), Error> {
        // this next line is pissing me off. I don't know what to do with it
        // keeps crashing the program instead of returning an error and moving
        // on with the rest of the program to circumvent the absence of the file
        // it is looking for.
        let text = fs::read_to_string(file_name);
        println!("!!!!!!!!!text: {:#?}", &text);

        // not sure what to do in here

        // let cfg: Config = from_str(&text)?;
        // println!("access_token: {}", self.access_token);

        if self.access_token != "" {
            println!("access_token: {}", self.access_token);
        } else {
            println!("no access token")
        }

        Ok(())
    }
}
