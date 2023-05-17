use clap::Parser;

use std::{fs::OpenOptions, thread::available_parallelism};
use text_analyzer::analyzer::{AnalyzerManager, AnalyzerResult};

mod ui;

#[derive(Parser)]
#[command(version, long_about=None)]
struct Args {
    /// Path to file
    path: String,
    /// Number of threads to use, uses all available threads by default
    #[arg(short)]
    thread_count: Option<usize>,
    /// How many bytes each thread will try to process at a time
    #[arg(short, default_value_t = 1024)]
    block_size: usize,
}

fn main() {
    let args = Args::parse();
    println!("Opening: {:?}", &args.path);
    let file = OpenOptions::new()
        .read(true)
        .open(&args.path)
        .expect("Failed to open file");

    let mut result = AnalyzerResult::new();
    result.source_name = args.path;
    result += AnalyzerManager::new(
        args.thread_count
            .unwrap_or_else(|| available_parallelism().unwrap().get()),
        args.block_size,
        file,
    )
    .analyze();
    println!("{result}");
}
