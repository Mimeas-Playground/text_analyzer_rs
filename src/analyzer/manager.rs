use std::{
    eprintln,
    io::{self, ErrorKind, Read, Seek},
    println,
    sync::{Arc, Mutex},
    thread,
};

use super::AnalyzerResult;

pub struct AnalyzerManager<S: Read + Seek> {
    thread_count: usize,
    thread_block_size: usize,
    text_stream: S,
    text_stream_length: Option<u64>,
}

impl<S: Read + Seek> AnalyzerManager<S> {
    pub fn new(thread_count: usize, thread_block_size: usize, text_stream: S) -> Self {
        let mut manager = Self {
            thread_count,
            thread_block_size,
            text_stream,
            text_stream_length: None,
        };

        if let Ok(bytes) = manager.text_stream.seek(io::SeekFrom::End(0)) {
            println!("AnalyzerManager: file has {bytes} bytes");
            manager
                .text_stream
                .seek(io::SeekFrom::Start(0))
                .expect("Failed to set curso to start");
            manager.text_stream_length = Some(bytes);
        } else {
            println!("AnalyzerManager: stream has no defined end");
        }

        manager
    }

    pub fn analyze(mut self) -> AnalyzerResult {
        let mut result = AnalyzerResult::new();

        // 1. Create txt buffer channel
        let (snd_buffer, rcv_buffer) = std::sync::mpsc::channel::<String>();
        let rcv_buffer = Arc::new(Mutex::new(rcv_buffer));
        // 2. Create worker threads
        thread::scope(|s| {
            let mut threads = Vec::with_capacity(self.thread_count);

            for id in 0..self.thread_count {
                let rcv_buffer = rcv_buffer.clone();

                threads.push(s.spawn(move || {
                    let id = id;
                    let mut result = AnalyzerResult::new();
                    loop {
                        let txt_query = rcv_buffer.lock().unwrap().recv();

                        if let Ok(buffer) = txt_query {
                            eprintln!("Worker {id} received buffer");
                            let buffer = buffer.split_whitespace().map(|s| s.to_string());
                            result.analyze(buffer);
                        } else {
                            break;
                        }
                    }

                    result
                }));
            }
            // 3. Load text buffer and send through channel

            let mut total_read = 0;
            loop {
                let mut buffer = vec![0; self.thread_block_size];

                match self.text_stream.read(&mut buffer) {
                    Ok(bytes) => {
                        if let Ok(mut txt) = String::from_utf8(buffer) {
                            total_read += bytes;

                            // A word that ends without a space may not be complete, therefore we
                            // continue to read until a whitespace is found or the stream is empty
                            if !txt.ends_with(char::is_whitespace) && bytes == total_read {
                                let mut bytes: Vec<u8> = Vec::new();
                                let mut byte = [0];
                                loop {
                                    match self.text_stream.read(&mut byte) {
                                        Ok(b) => {
                                            if b == 0 {
                                                break;
                                            }
                                            bytes.extend_from_slice(&byte);
                                            total_read += 1;

                                            // If this fails to interpret as utf8 then the character is
                                            // not complete yet
                                            if let Ok(word) = std::str::from_utf8(bytes.as_slice())
                                            {
                                                if word.ends_with(char::is_whitespace) {
                                                    break;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            // Reached end of stream
                                            if e.kind() == std::io::ErrorKind::WouldBlock
                                                || e.kind() == std::io::ErrorKind::UnexpectedEof
                                            {
                                                break;
                                            } else {
                                                // An actual error
                                                panic!("Error reading from stream: {}", e);
                                            }
                                        }
                                    }
                                }

                                if let Ok(word) = std::str::from_utf8(bytes.as_slice()) {
                                    txt.push_str(word);
                                }
                            }

                            snd_buffer.send(txt).unwrap();
                            eprintln!("Sendt {bytes} long buffer to threads");

                            // This is the end of the stream, last
                            // word will never complete if not
                            // already
                            if bytes != self.thread_block_size {
                                println!("Finished loading stream");
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::Interrupted {
                            continue;
                        } else {
                            eprintln!("Encountered while reading file {e}");
                        }
                    }
                }

                if let Some(length) = self.text_stream_length {
                    print!(
                        "\rAnalyzed {:.2}% of file\t",
                        (total_read as f64 / length as f64) * 100.0
                    );
                } else {
                    println!("Analyzed {} bytes", total_read);
                }
            }

            // 4. when file is empty, close txt buffer channel
            // 5. join worker threads and combine results
            // 6. return result

            drop(snd_buffer);

            for worker in threads {
                result += worker.join().unwrap();
            }
        });
        result
    }
}
