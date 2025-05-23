fn main() {
    const N_THREADS: usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let mut thread_handles: Vec<std::thread::JoinHandle<u32>> = Vec::new();
    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        thread_handles.push(std::thread::spawn(move || {
            my_chunk.iter().sum()
        }));
    }

    let mut sum: u32 = 0;
    for handle in thread_handles {
        sum += handle.join().unwrap();
    }
    println!("Sum is {sum}");
}
