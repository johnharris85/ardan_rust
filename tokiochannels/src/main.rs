use std::{time::Duration, sync::mpsc};

enum Command {
    Print(String),
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(s) => println!("{s}"),
            }
        }
    });

    let mut counter = 0;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Hello {counter}"))).unwrap();
        counter += 1;
    }
}
