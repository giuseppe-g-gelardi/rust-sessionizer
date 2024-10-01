use std::error::Error;

mod auth;
mod cli;
mod config;
mod env;
mod repo;

use auth::auth::authenticate;
use cli::cli::init;
use config::config::CfgManager;
use env::env::load_env;
use repo::repo::get_repos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = load_env();
    let config_manager = CfgManager::new_cfg_manager();
    let config = &config_manager.get_config(1)?;
    let config_exists = &config_manager.verify_config_exists();

    if *config_exists && config.access_token.is_empty() {
        authenticate(env.client_id, env.client_secret, &config_manager, &config).await?;
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("You are already authenticated... starting CL!");
    println!("access_token: {:?}", config);

    let repos = get_repos(config.access_token.to_string()).await?;

    init(&config_manager, repos);

    Ok(())
}
