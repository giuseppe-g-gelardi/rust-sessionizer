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


//
// use octocrab::{models::Repository, Octocrab};
// use std::error::Error;
//
// pub async fn get_repos(token: String) -> Result<(), Box<dyn Error>> {
//     println!("token: {:?}", token);
//     let o = Octocrab::builder().personal_token(token).build()?;
//
//     let mut all_repos: Vec<Repository> = Vec::new();
//     let mut page = 1;
//
//     loop {
//         // Fetch repositories for the current page
//         let repos = o
//             .current()
//             .list_repos_for_authenticated_user()
//             .per_page(100) // Maximum number of repositories per page
//             .page(page)
//             .send()
//             .await?;
//
//         if repos.is_empty() {
//             break; // Exit the loop if no more repositories are found
//         }
//
//         all_repos.extend(repos); // Add the fetched repositories to the list
//         page += 1; // Increment to fetch the next page
//     }
//
//     // Process and print the fetched repositories
//     all_repos.into_iter().for_each(|r: Repository| {
//         let is_private = r.private.unwrap_or(false);
//         let visibility = if is_private { "private" } else { "public" };
//         
//         let description = r.description.as_deref().unwrap_or("").chars().take(15).collect::<String>();
//         
//         println!("{:?} ({}) {:?}", r.name, visibility, description);
//     });
//
//     Ok(())
// }
//
