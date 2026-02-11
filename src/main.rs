mod export;
mod parser;
mod summary;
mod worker;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex, mpsc},
    thread,
};

use chrono::NaiveDate;
use export::export_csv;
use summary::Summary;
use worker::worker;

const DEFAULT_WORKER_COUNT: usize = 4;
const CHUNK_SIZE: usize = 1000;

fn main() {
    let file = File::open("app.log").expect("Cannot open log file");
    let reader = BufReader::new(file);

    let summary = Arc::new(Mutex::new(Summary::default()));
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let date_filter = NaiveDate::from_ymd_opt(2024, 1, 1);
    let level_filter = None; // Some("ERROR".to_string());

    let mut handles = Vec::new();
    let worker_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(DEFAULT_WORKER_COUNT);

    for _ in 0..worker_count {
        let rx = Arc::clone(&rx);
        let summary = Arc::clone(&summary);
        let level_filter = level_filter.clone();

        handles.push(thread::spawn(move || {
            worker(rx, summary, date_filter, level_filter)
        }));
    }

    // Producer: read file by chunk
    let mut buffer = Vec::with_capacity(CHUNK_SIZE);
    for line in reader.lines() {
        buffer.push(line.unwrap());

        if buffer.len() == CHUNK_SIZE {
            tx.send(buffer).unwrap();
            buffer = Vec::with_capacity(CHUNK_SIZE);
        }
    }

    if !buffer.is_empty() {
        tx.send(buffer).unwrap();
    }

    drop(tx); // Important: close channel

    for handle in handles {
        handle.join().unwrap();
    }

    // Export summary
    let summary = summary.lock().unwrap();
    println!("Log Summary:");
    for (level, count) in &summary.level_counts {
        println!("{}: {}", level, count);
    }

    export_csv(&summary);
}
