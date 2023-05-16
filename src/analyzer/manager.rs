use std::{
    fs::File,
    io::Seek,
    ops::Deref,
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

            let stopsignal = Arc::new(Mutex::new(false));

            let thr_args = (self.text_stream.clone(), stopsignal.clone());
            s.spawn(move || {
                while !thr_args.1.lock().unwrap().deref() {
                    if let Ok(mut stream) = thr_args.0.lock() {
                        let curr = stream.stream_position().unwrap();
                        let len = stream.metadata().unwrap().len();
                        let progress = (curr as f64 / len as f64) * 100.0;
                        println!("Progress: {:.2}%", progress);
                    }

                    thread::sleep(std::time::Duration::from_millis(70));
                }
            });

            for (i, join) in threads.into_iter().enumerate() {
                println!("Waiting for thread: {i}");
                result += join.join().unwrap();
            }

            *stopsignal.lock().unwrap() = true;
        });

        result
    }
}
