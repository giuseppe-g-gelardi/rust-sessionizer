// use octocrab::models::events::Repository;
use octocrab::{models::Repository, Octocrab};
use std::error::Error;

pub async fn get_repos(token: String) -> Result<(), Box<dyn Error>> {
    println!("token: {:?}", token);
    let o = Octocrab::builder().personal_token(token).build()?;
    let x = o
        .current()
        .list_repos_for_authenticated_user()
        .send()
        .await?;
    //
    // let repos = octocrab.repos("giuseppe-g-gelardi", "rust-sessionizer").get().await?;
    //
    // println!("repos: {:?}", repos);

    println!("repos: {:?}", x);

    Ok(())
}

// var API_URL = "https://api.github.com/user/repos?page={PAGE}&per_page={PER_PAGE}&visibility=all"
