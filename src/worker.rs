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
        let lines = {
            let rx = rx.lock().unwrap();
            rx.recv()
        };

        let lines = match lines {
            Ok(l) => l,
            Err(_) => break,
        };

        for line in lines {
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
    for (level, count) in local_counts {
        *global.level_counts.entry(level).or_insert(0) += count;
    }
}
