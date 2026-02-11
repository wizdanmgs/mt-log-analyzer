use chrono::NaiveDate;

#[derive(Debug)]
pub struct LogEntry {
    pub date: NaiveDate,
    pub level: String
}

pub fn parse_line(line: &str) -> Option<LogEntry> {
    let mut parts = line.splitn(3, ' ');
    let date = parts.next()?;
    let level = parts.next()?;

    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;

    Some(LogEntry { date, level: level.to_string() })
}
