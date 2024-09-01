// internal
use config::config::Config;

// crates
// use serde_json::from_str;

// standard library
use std::fs;
use std::io::Error;
use std::path::Path;

// modules
mod config;

// help me
use std::env;
// use std::fs;
use std::path::PathBuf;
// use std::os::unix::fs::MetadataExt; // For Linux-specific extensions
use dirs; // `dirs` crate helps in fetching the user's home directory and application data

const FILE_NAME: &str = "cfg.json";

fn main() -> Result<(), Error> {
    let loc = get_config_file_location();
    match loc {
        Some(path) => println!("Config file location: {:?}", path),
        None => eprintln!("Failed to get config file location"),
    }

    // let text = fs::read_to_string(file_name.to_string())?;
    let file_name: Vec<String> = std::env::args().collect();
    let file_exists = verify_config_exists(&file_name[1]);

    let mut cfg = Config::new();

    cfg.update_config_file(&FILE_NAME.to_string()); // update_config_file();

    verify_file_exists(&cfg, file_name, file_exists)?; // verify_file_exists(&cfg, file_name,
                                                       // file_exists);

    // match file_exists {
    //     Ok(_) => {
    //         println!("file exists");
    //         cfg.verify_access_token(&file_name[1]).unwrap();
    //     }
    //     Err(e) => {
    //         println!(
    //             "file does not exist, creating new config file. {:?}",
    //             e.to_string()
    //         );
    //         cfg.create_config(&FILE_NAME.to_string()).unwrap();
    //     }
    // }

    Ok(())
}

pub fn get_config_file_location() -> Option<PathBuf> {
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

    // Create "session_config.yaml" path
    Some(config_dir.join("session_config.yaml"))
}

fn verify_file_exists(
    cfg: &Config,
    file_name: Vec<String>,
    file_exists: Result<(), Error>,
) -> Result<(), Error> {
    match file_exists {
        Ok(_) => {
            println!("file exists");
            cfg.verify_access_token(&file_name[1]).unwrap();
        }
        Err(e) => {
            println!(
                "file does not exist, creating new config file. {:?}",
                e.to_string()
            );
            cfg.create_config(&FILE_NAME.to_string()).unwrap();
        }
    }

    Ok(())
}

fn verify_config_exists(file_name: &String) -> Result<(), Error> {
    if Path::new(file_name).exists() {
        fs::read_to_string(file_name)?; // let text = ????? // need to get rid of this ? lol
        Ok(())
    } else {
        Err(Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
}

// fn create_config(file_name: &String) -> Result<(), Error> {
//     let cfg = Config::new();
//     cfg.save_to_file(file_name)?;
//     Ok(())
// }

// fn verify_access_token(file_name: &String) -> Result<(), Error> {
//     let text = fs::read_to_string(file_name.to_string())?;
//
//     let cfg: Config = from_str(&text)?;
//
//     println!("access_token: {}", cfg.access_token);
//     Ok(())
// }

// fn update_config_file() {
//     let mut cfg = Config::new();
//
//     cfg.access_token = "1234567890".to_string();
//     cfg.editor = "VSCode".to_string();
//     cfg.alias = None;
//     cfg.tmux = true;
//
//     cfg.save_to_file("cfg.json").unwrap();
// }
