use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
    thread,
};

use super::{AnalyzerResult, AnalyzerThread};

pub struct AnalyzerManager {
    thread_count: usize,
    text_stream: Arc<Mutex<File>>,
}

impl AnalyzerManager {
    pub fn new(thread_count: usize, text_stream: File) -> Self {
        println!(
            "AnalyzerManager: file has {} bytes",
            text_stream.metadata().unwrap().len()
        );
        Self {
            thread_count,
            text_stream: Arc::new(Mutex::new(text_stream)),
        }
    }

    pub fn analyze(self) -> AnalyzerResult {
        let mut result = AnalyzerResult::new();

        thread::scope(|s| {
            let mut threads = Vec::new();

            for i in 0..self.thread_count {
                let thr = AnalyzerThread::new(self.text_stream.clone());
                println!("Creating worker thread: {i}");

                threads.push(s.spawn(|| thr.analyze()));
            }

            for (i, join) in threads.into_iter().enumerate() {
                println!("Waiting for thread: {i}");
                result += join.join().unwrap();
            }
        });

        result
    }
}
