use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub fn run() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];
    let start = Instant::now();

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut local_sum = 0u64;
            for _ in 0..1_000 {
                local_sum += 1;
            }

            let mut num = c.lock().unwrap();
            *num += local_sum;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());
    println!("Time elapsed: {:?}", start.elapsed());
}
