use std::fs::File;
use std::io::Write;

use crate::summary::Summary;

pub fn export_csv(summary: &Summary) {
    let mut file = File::create("summary.csv").unwrap();
    writeln!(file, "level,count").unwrap();

    for (level, count) in &summary.level_counts {
        writeln!(file, "{},{}", level, count).unwrap();
    }
}
