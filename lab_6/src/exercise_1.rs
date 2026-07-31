use std::thread;

pub fn run() {
    let mut handles = vec![];
    let chunk_size = 1000 / 4;

    for i in 0..4 {
        let start = i * chunk_size + 1;
        let end = (i + 1) * chunk_size;

        let handle = thread::spawn(move || {
            let sum: u32 = (start..=end).sum();
            println!("[thread {}] sum {}..={} = {}", i, start, end, sum);
            sum
        });
        handles.push(handle);
    }

    let total_sum: u32 = handles
        .into_iter()
        .map(|h| h.join().expect("Thread panicked"))
        .sum();
    //
    println!("Total: {}", total_sum);
}
