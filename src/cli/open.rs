use crate::cli::cli::init;
use crate::config::config::CfgManager;
use crate::repo::repo::PartialRepo;

// use dialoguer::Select;

pub fn open(cm: &CfgManager, repos: Vec<PartialRepo>) {
    // let repo_names: Vec<String> = repos.iter().map(|r| r.name.clone()).collect();

    let repo_select_options = repos.iter().for_each(|r| {
        println!("{:?} {:?} {:?}", r.name, r.visibility, r.description);
    });

    println!("!!!!!REPO_NAMES: {:?}", repo_select_options);

    init(cm, repos)
}
