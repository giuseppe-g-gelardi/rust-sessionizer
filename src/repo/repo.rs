// use octocrab::models::events::Repository;
use octocrab::{models::Repository, Octocrab};
use std::error::Error;

/*
 * NOTE: 
 * update this to fetch ALL repos, not just the first page
 *
 * use the go project as a reference
 */

pub async fn get_repos(token: String) -> Result<(), Box<dyn Error>> {
    let o = Octocrab::builder().personal_token(token).build()?;
    let x = o
        .current()
        .list_repos_for_authenticated_user()
        .send()
        .await?;

    x.into_iter().for_each(|r: Repository| {
        let is_private = r.private.unwrap_or(false);
        let visibility = if is_private { "private" } else { "public" };
        let description = r
            .description
            .as_deref()
            .unwrap_or("")
            .chars()
            .take(20)
            .collect::<String>();
        println!("{:?} ({:?}) {:?}", r.name, visibility, description);
    });

    Ok(())
}
