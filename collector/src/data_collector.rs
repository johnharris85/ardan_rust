use shared_data::CollectorCommandV1;
use std::{time::Instant, sync::mpsc::Sender};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect to the server")]
    UnableToConnect,
    #[error("Unable to send data to the server")]
    UnableToSend,
    #[error("Unable to receive data from the server")]
    UnableToReceive,
}

pub fn collect_data(tx: Sender<CollectorCommandV1>, collector_id: u128) {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    loop {
        let now = Instant::now();

        // Refresh the stored data
        sys.refresh_memory();
        sys.refresh_cpu_all();

        // Get new values
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let num_cpus = sys.cpus().len();
        let total_cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();
        let average_cpu_usage = total_cpu_usage / num_cpus as f32;

        // Submit
        let send_result = tx.send(CollectorCommandV1::SubmitData {
            collector_id,
            total_memory,
            used_memory,
            average_cpu_usage,
        });
        if let Err(e) = send_result {
            println!("Error sending data: {e:?}");
        }

        // Wait for the next cycle
        let elapsed_seconds = now.elapsed().as_secs_f32();
        if elapsed_seconds < 1.0 {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 - elapsed_seconds));
        } else {
            // Warning: we're running behind!
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
        }
    }
}