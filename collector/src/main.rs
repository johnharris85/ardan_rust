use std::collections::VecDeque;

use sender::send_queue;
use shared_data::{encode_v1, CollectorCommandV1};
mod data_collector;
mod sender;

fn get_uuid() -> u128 {
    let path = std::path::Path::new("uuid");
    if path.exists() {
        let contents = std::fs::read_to_string(path).unwrap();
        contents.parse::<u128>().unwrap()
    } else {
        let uuid = uuid::Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string()).unwrap();
        uuid
    }
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();
    let uuid = get_uuid();
    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx, uuid);
    });

    let mut data_queue = VecDeque::with_capacity(120);

    // Listen for commands to send
    while let Ok(command) = rx.recv() {
        let encoded = encode_v1(&command);
        println!("Encoded {} bytes", encoded.len());
        data_queue.push_back(encoded);
        let _ = send_queue(&mut data_queue);
    }
}
