use crate::config::config::CfgManager;
use crate::repo::repo::get_repos;
use std::error::Error;

use dialoguer::Select;

pub async fn open(cm: &CfgManager) -> Result<(), Box<dyn Error>> {

    let token = cm.get_config(1).unwrap().access_token.to_string();
    let repos = get_repos(token).await?;
    let repo_names: Vec<String> = repos.iter().map(|r| r.name.clone()).collect();

    println!("!!!!!REPO_NAMES: {:?}", repo_names);

    Ok(())
}
