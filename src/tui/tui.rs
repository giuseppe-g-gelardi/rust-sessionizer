use dialoguer::Select;

enum Welcome {
    Open,
    Update,
    Exit,
}

impl Welcome {
    fn execute(&self) -> String {
        match self {
            Welcome::Open => open(),
            Welcome::Update => update(),
            Welcome::Exit => exit(),
        }
    }
}

pub fn welcome_dialog() -> String {
    let choices = vec!["Open", "Update", "Exit"];

    let selections = Select::new()
        .items(&choices)
        .default(0)
        .with_prompt(
            "\nWould you like to:\n- Open a repo?\n- Update your config?\n- Exit the program?\n"
        )
        .interact()
        .unwrap();

    match selections {
        0 => Welcome::Open.execute(),
        1 => Welcome::Update.execute(),
        2 => Welcome::Exit.execute(),
        _ => Welcome::Exit.execute(),
    }
}

pub fn open() -> String {
    "open function!!!!".to_string()
}

pub fn update() -> String {
    "update function!!!!".to_string()
}

pub fn exit() -> String {
    "exit function!!!!".to_string()
}
