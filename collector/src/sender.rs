use std::{collections::VecDeque, io::{Read, Write}};
use shared_data::{decode_response_v1, decode_v1, CollectorResponseV1, DATA_COLLECTOR_ADDRESS};

use crate::data_collector::CollectorError;

pub fn send_command(bytes: &[u8]) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream.write_all(bytes)
        .map_err(|_| CollectorError::UnableToSend)?;
    Ok(())
}

pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    // Connect
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;

    // Send every queue item
    let mut buf = vec![0u8; 512];
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSend);
        }
        let bytes_read = stream.read(&mut buf).map_err(|_| CollectorError::UnableToReceive)?;
        if bytes_read == 0 {
            queue.push_front(command);
            return Err(CollectorError::UnableToReceive);
        }
        let ack = decode_response_v1(&buf[0..bytes_read]);
        let ts = decode_v1(&command);
        if ack != CollectorResponseV1::Ack(ts.0) {
            queue.push_front(command);
            return Err(CollectorError::UnableToReceive);
        } else {
            // Comment this out for production
            println!("Ack received");
        }
    }

    Ok(())
}