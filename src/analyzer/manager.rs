use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
    thread,
};

use super::{AnalyzerResult, AnalyzerThread};

pub struct AnalyzerManager {
    thread_count: usize,
    text_stream: Arc<Mutex<BufReader<File>>>,
}

impl AnalyzerManager {
    pub fn new(thread_count: usize, text_stream: BufReader<File>) -> Self {
        Self {
            thread_count,
            text_stream: Arc::new(Mutex::new(text_stream)),
        }
    }

    pub fn analyze(self) -> AnalyzerResult {
        let mut result = AnalyzerResult::new();

        thread::scope(|s| {
            let mut threads = Vec::new();

            for _ in 0..self.thread_count {
                let thr = AnalyzerThread::new(self.text_stream.clone());

                threads.push(s.spawn(|| thr.analyze()));
            }

            for join in threads {
                result += join.join().unwrap();
            }
        });

        result
    }
}
