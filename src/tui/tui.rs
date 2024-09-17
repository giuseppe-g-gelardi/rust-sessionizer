use dialoguer::Select;

pub fn set_welcome() {
    // single select
    let choices = vec!["Open", "Update", "Exit"];

    let selections = Select::new().items(&choices).default(0).interact().unwrap();

    match selections {
        0 => println!("Opening..."),
        1 => println!("Updating..."),
        2 => println!("Exiting..."),
        _ => println!("Exiting..."),
    }
}
