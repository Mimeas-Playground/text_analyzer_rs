use std::{
    fs::File,
    io::{self, Seek, Write},
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
                let thr = AnalyzerThread::with_block_size(
                    self.thread_block_size,
                    self.text_stream.clone(),
                );
                println!("Creating worker thread: {i}");

                threads.push(s.spawn(|| thr.analyze()));
            }

            let (tx_stop, rx_stop) = std::sync::mpsc::channel(); // Create a channel to send a stop signal
            let thr_args = (self.text_stream.clone(), rx_stop);
            s.spawn(move || {
                while thr_args.1.try_recv().is_err() {
                    if let Ok(mut stream) = thr_args.0.lock() {
                        let curr = stream.stream_position().unwrap();
                        let len = stream.metadata().unwrap().len();
                        drop(stream);

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

            tx_stop.send(true).unwrap();
        });

        result
    }
}
