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

const WORKER_COUNT: usize = 4;

fn main() {
    let file = File::open("app.log").expect("Cannot open log file");
    let reader = BufReader::new(file);

    let summary = Arc::new(Mutex::new(Summary::default()));
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let date_filter = NaiveDate::from_ymd_opt(2026, 2, 10);
    let level_filter = None; // Some("ERROR".to_string());

    let mut handles = Vec::new();

    for _ in 0..WORKER_COUNT {
        let rx = Arc::clone(&rx);
        let summary = Arc::clone(&summary);
        let level_filter = level_filter.clone();

        handles.push(thread::spawn(move || {
            worker(rx, summary, date_filter, level_filter)
        }));
    }

    // Producer: read file line by line
    for line in reader.lines() {
        tx.send(line.unwrap()).unwrap();
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
