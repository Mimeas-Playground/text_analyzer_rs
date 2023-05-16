use std::{
    fs::File,
    io::{self, Seek, Write},
    ops::Deref,
    sync::{Arc, Mutex},
    thread,
};

use super::{AnalyzerResult, AnalyzerThread};

pub struct AnalyzerManager {
    thread_count: usize,
    thread_block_size: usize,
    text_stream: Arc<Mutex<File>>,
}

impl AnalyzerManager {
    pub fn new(thread_count: usize, thread_block_size: usize, text_stream: File) -> Self {
        println!(
            "AnalyzerManager: file has {} bytes",
            text_stream.metadata().unwrap().len()
        );
        Self {
            thread_count,
            thread_block_size,
            text_stream: Arc::new(Mutex::new(text_stream)),
        }
    }

    pub fn analyze(self) -> AnalyzerResult {
        let mut result = AnalyzerResult::new();

        thread::scope(|s| {
            let mut threads = Vec::new();

            for i in 0..self.thread_count {
                let thr = AnalyzerThread::with_block_size(1024, self.text_stream.clone());
                println!("Creating worker thread: {i}");

                threads.push(s.spawn(|| thr.analyze()));
            }

            // Weird implementation of a progress message
            // There shold be a better way to do this TODO
            let stopsignal = Arc::new(Mutex::new(false));
            let thr_args = (self.text_stream.clone(), stopsignal.clone());
            s.spawn(move || {
                while !thr_args.1.lock().unwrap().deref() {
                    if let Ok(mut stream) = thr_args.0.lock() {
                        let curr = stream.stream_position().unwrap();
                        let len = stream.metadata().unwrap().len();
                        let progress = (curr as f64 / len as f64) * 100.0;
                        print!("\rProgress: {:.2}%\t", progress);
                        io::stdout().flush().unwrap();
                    }

                    thread::sleep(std::time::Duration::from_millis(100));
                }
            });

            for join in threads {
                result += join.join().unwrap();
            }

            *stopsignal.lock().unwrap() = true;
        });

        result
    }
}
