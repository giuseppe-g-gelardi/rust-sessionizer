use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

pub fn exit() {
    let mut message = "Exiting".to_string();
    let sleep_duration = Duration::from_secs(1);

    for _ in 0..4 {
        print!("{}\r", message);
        io::stdout().flush().unwrap();
        thread::sleep(sleep_duration);
        message.push_str(".");
    }

    // NOTE: print! and println! are very different
    print!(" Bye! 👋");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(2));
}
