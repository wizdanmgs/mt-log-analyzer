use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "mtla")]
#[command(about = "Multi-threaded log analyzer", long_about = None)]
pub struct Cli {
    /// Path to log file
    #[arg(short, long)]
    pub input: String,

    /// Filter by date (YYYY-MM-DD)
    #[arg(long)]
    pub date: Option<String>,

    /// Filter by log level
    #[arg(long)]
    pub level: Option<String>,

    /// Number of worker threads
    #[arg(short, long)]
    pub workers: Option<usize>,

    /// Export summary to CSV
    #[arg(long)]
    pub export: Option<String>,

    /// Chunk size
    #[arg(long, default_value_t = 1000)]
    pub chunk_size: usize,
}
