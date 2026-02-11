mod cli;
mod error;
mod parser;
mod summary;
mod worker;

use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    sync::{Arc, Mutex, mpsc},
    thread,
};

use chrono::NaiveDate;
use clap::Parser;

use cli::Cli;
use error::AppError;
use summary::Summary;
use worker::worker;

fn main() -> Result<(), AppError> {
    let cli = Cli::parse();
    let worker_count = cli.workers.unwrap_or_else(|| {
        thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    });

    let date_filter = if let Some(date_str) = cli.date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| AppError::InvalidDate)?)
    } else {
        None
    };

    let file = File::open(&cli.input)?;
    let reader = BufReader::new(file);

    // Bounded channel (prevents memory explosion)
    let (tx, rx) = mpsc::sync_channel::<Vec<String>>(worker_count * 2);

    let rx = Arc::new(Mutex::new(rx));
    let summary = Arc::new(Mutex::new(Summary::default()));

    let mut handles = Vec::new();

    for _ in 0..worker_count {
        let rx = Arc::clone(&rx);
        let summary = Arc::clone(&summary);
        let date_filter = date_filter.clone();
        let level_filter = cli.level.clone();

        handles.push(thread::spawn(move || {
            worker(rx, summary, date_filter, level_filter)
        }));
    }

    // Producer: read file by chunk
    let mut buffer = Vec::with_capacity(cli.chunk_size);

    for line in reader.lines() {
        buffer.push(line?);

        if buffer.len() == cli.chunk_size {
            tx.send(buffer).unwrap();
            buffer = Vec::with_capacity(cli.chunk_size);
        }
    }

    if !buffer.is_empty() {
        tx.send(buffer).unwrap();
    }

    drop(tx); // Important: close channel

    for handle in handles {
        handle.join().unwrap();
    }

    let summary = summary.lock().unwrap();

    println!("=== Log Summary ===");
    for (level, count) in &summary.level_counts {
        println!("{level}: {count}");
    }

    if let Some(path) = cli.export {
        let mut file = File::create(path)?;
        writeln!(file, "level,count")?;

        for (level, count) in &summary.level_counts {
            writeln!(file, "{},{}", level, count)?;
        }
    }

    Ok(())
}
