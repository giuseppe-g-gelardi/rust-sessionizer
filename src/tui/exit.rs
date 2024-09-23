use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn exit() -> String {
    let mut message = "Exiting".to_string();
    let sleep_duration = Duration::from_secs(1);

    for _ in 0..4 {
        print!("{}\r", message);
        io::stdout().flush().unwrap();
        thread::sleep(sleep_duration);
        message.push_str(".");
    }

    print!(" Bye! 👋");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(2));

    // "exit function!!!!".to_string()
    "".to_string()
}


// NOTE: print! and println! are very different
