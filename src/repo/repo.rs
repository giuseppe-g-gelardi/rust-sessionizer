use octocrab::{models::Repository, Octocrab};
use std::error::Error;

#[derive(Debug)]
pub struct PartialRepo {
    pub name: String,
    pub html_url: String,
    pub ssh_url: String,
    pub description: String,
    pub visibility: String,
}

pub async fn get_repos(token: String) -> Result<Vec<PartialRepo>, Box<dyn Error>> {
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let mut all_repos: Vec<Repository> = Vec::new();
    let mut page = 1;

    loop {
        let repos = octocrab
            .current()
            .list_repos_for_authenticated_user()
            .per_page(100)
            .page(page)
            .send()
            .await?;

        if repos.items.is_empty() {
            break;
        }

        all_repos.extend(repos); // Add the fetched repositories to the list
        page += 1; // Increment to fetch the next page
    }

    let partial_repos = generate_partial_repos(&all_repos);

    Ok(partial_repos)
}

fn generate_partial_repos(repos: &Vec<Repository>) -> Vec<PartialRepo> {
    let mut partial_repos: Vec<PartialRepo> = Vec::new();
    repos
        .iter()
        .enumerate()
        .for_each(|(_index, r): (usize, &Repository)| {
            let is_private = r.private.unwrap_or(false);
            let visibility = if is_private { "private" } else { "public" };

            let description = r
                .description
                .as_deref()
                .unwrap_or("")
                .chars()
                .take(15)
                .collect::<String>();

            let html_url = r.html_url.as_ref().expect("html_url not found");
            let ssh_url = r.ssh_url.as_ref().expect("ssh_url not found");

            let partial_repo = PartialRepo {
                name: r.name.clone(),
                html_url: html_url.to_string(),
                ssh_url: ssh_url.to_string(),
                description,
                visibility: visibility.to_string(),
            };

            partial_repos.push(partial_repo);
        });

    partial_repos
}
