# Multi-Threaded Log Analyzer

A high-performance, multi-threaded CLI tool for analyzing large log files.

Built using:

- `std::thread`
- `std::sync::mpsc`
- `Arc`
- `Mutex`
- `clap`
- `chrono`

Designed to process multi-GB log files efficiently while maintaining Rust’s safety guarantees.

---

## Features

- Multi-threaded processing
- Filter by date (`YYYY-MM-DD`)
- Filter by log level (`INFO`, `WARN`, `ERROR`, etc.)
- Count occurrences per level
- Export summary to CSV
- Bounded channel (prevents memory explosion)
- Chunked processing (performance optimized)

---

## Log Format

Each log entry must follow:

```
YYYY-MM-DD LEVEL Message
```

Example:

```
2024-01-01 INFO User logged in
2024-01-01 ERROR Database timeout occurred
2024-01-01 WARN Memory usage high
```

Rules:

- Date must be the **first word**
- Entries must be **one per line**
- Sorted by date (recommended but not required)

---

## Installation

Clone the project:

```bash
git clone https://github.com/wizdanmgs/mt-log-analyzer
cd mt-log-analyzer
```

Build:

```bash
cargo build --release
```

Binary will be located at:

```
target/release/mtla
```

---

## Usage

### Basic Run

```bash
cargo run -- --input app.log
```

Or using the compiled binary:

```bash
./target/release/mtla --input app.log
```

---

### Filter by Date

```bash
mtla --input app.log --date 2024-06-01
```

---

### Filter by Level

```bash
mtla --input app.log --level ERROR
```

---

### Set Worker Count

```bash
mtla --input app.log --workers 8
```

If not specified, it defaults to:

```
Number of available CPU cores
```

---

### Change Chunk Size

Controls how many lines are sent to workers per batch.

```bash
mtla --input app.log --chunk-size 2000
```

Default: `1000`

---

### Export Summary to CSV

```bash
mtla --input app.log --export summary.csv
```

Output format:

```
level,count
INFO,5234
ERROR,321
WARN,872
```

---

## Architecture Overview

```
Reader Thread (Producer)
        │
        ▼
Bounded Channel (sync_channel)
        │
        ▼
Arc<Mutex<Receiver>>
        │
        ▼
Worker Threads
        │
        ▼
Local Aggregation (HashMap per worker)
        │
        ▼
Global Merge (Arc<Mutex<Summary>>)
```

---

## Performance Design

### Local Aggregation

Each worker maintains its own `HashMap` to reduce lock contention.

Instead of:

```
Lock per log line ❌
```

We use:

```
Lock once per worker ✅
```

---

### Chunked Processing

Instead of sending 1 line per message:

```
10,000,000 recv() calls ❌
```

We send batches:

```
10,000 recv() calls ✅
```

---

### Bounded Channel

Uses `sync_channel` to:

- Prevent memory explosion
- Provide backpressure
- Keep memory stable for large files

---

## Performance Expectations

| Log Size      | Expected Performance     |
| ------------- | ------------------------ |
| 150k lines    | Instant                  |
| 1M lines      | < 1 second               |
| Multi-GB logs | Stable memory, CPU-bound |

Performance depends on:

- CPU cores
- Disk speed
- Chunk size
- Log format complexity
