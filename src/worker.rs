use chrono::NaiveDate;
use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::parser::parse_line;
use crate::summary::Summary;

pub fn worker(
    rx: Arc<Mutex<Receiver<String>>>,
    summary: Arc<Mutex<Summary>>,
    date_filter: Option<NaiveDate>,
    level_filter: Option<String>,
) {
    loop {
        let line = {
            let rx = rx.lock().unwrap();
            rx.recv()
        };

        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

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

            let mut summary = summary.lock().unwrap();
            summary.increment(&entry.level);
        }
    }
}
