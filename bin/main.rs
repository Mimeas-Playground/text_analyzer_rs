use itertools::Itertools;
use std::{fs::OpenOptions, path::PathBuf};
use text_analyzer::analyzer::AnalyzerManager;

mod ui;

fn main() {
    let mut thread_num = 0;
    let mut block_size = 1024;
    let mut path = PathBuf::new();
    let mut arg_valid = false;

    let mut args = std::env::args().skip(1);

    if let Some(file) = args.next() {
        path.push(file);
        arg_valid = path.is_file();
    }

    if args.len() > 0 {
        for (a, b) in std::env::args().tuple_windows() {
            if arg_valid == false {
                break;
            }

            if a == "-t" {
                if let Ok(num) = b.parse() {
                    thread_num = num;
                } else {
                    arg_valid = false;
                }
            } else if a == "-b" {
                if let Ok(num) = b.parse() {
                    block_size = num;
                } else {
                    arg_valid = false;
                }
            }
        }
    }

    if arg_valid {
        println!("Opening: {:?}", path);
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("Failed to open file");

        println!("File: {:?}", file);

        if thread_num == 0 {
            thread_num = std::thread::available_parallelism().unwrap().get()
        }
        let result = AnalyzerManager::new(thread_num, block_size, file).analyze();
        println!("Result:\n{}", result);
    } else {
        println!(
            "Usage: {} <file> [options]",
            std::env::args().nth(0).unwrap()
        );
        println!("\t -t <num>\t Number of threads to use");
        println!("\t -b <num>\t How many bytes each thread will try to process at a time");
    }
}
