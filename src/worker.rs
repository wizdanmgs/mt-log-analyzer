use chrono::NaiveDate;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::parser::parse_line;
use crate::summary::Summary;

pub fn worker(
    rx: Arc<Mutex<Receiver<Vec<String>>>>,
    summary: Arc<Mutex<Summary>>,
    date_filter: Option<NaiveDate>,
    level_filter: Option<String>,
) {
    let mut local_counts: HashMap<String, usize> = HashMap::new();

    loop {
        let chunk = {
            let rx = rx.lock().unwrap();
            rx.recv()
        };

        let chunk = match chunk {
            Ok(c) => c,
            Err(_) => break,
        };

        for line in chunk {
            if let Some(entry) = parse_line(&line) {
                if let Some(date) = date_filter
                    && entry.date != date
                {
                    continue;
                }

                if let Some(ref level) = level_filter
                    && &entry.level != level
                {
                    continue;
                }

                *local_counts.entry(entry.level).or_insert(0) += 1;
            }
        }
    }

    // Merge once at the end
    let mut global = summary.lock().unwrap();
    global.merge(local_counts);
}
