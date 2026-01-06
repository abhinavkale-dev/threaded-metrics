use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Metrics{
    total_ops: u64,
    per_thread: Vec<u64>,
}

fn main() {
    let thread_count = 4;
    let ops_per_thread = 1_000_000;

    let metrics = Arc::new(Mutex::new(Metrics {
        total_ops: 0,
        per_thread: vec![0; thread_count],
    }));
    let mut handles = Vec::new();

    for thread_id in 0..thread_count {
        let metrics_clone = Arc::clone(&metrics);

        let handle = thread::spawn(move || {
            for _ in 0..ops_per_thread {
                let mut metrics = metrics_clone.lock().unwrap();
                metrics.total_ops += 1;
                metrics.per_thread[thread_id] += 1;
            }
        println!("Thread {} finished", thread_id);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let metrics = metrics.lock().unwrap();

    println!("Total ops {}", metrics.total_ops);

    for(i, count) in metrics.per_thread.iter().enumerate() {
        println!("Thread {} operations: {}", i, count);
    }
}