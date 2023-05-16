use super::AnalyzerResult;
use std::{
    collections::vec_deque::VecDeque,
    io::Read,
    sync::{Arc, Mutex},
};

pub struct AnalyzerThread<S: Read> {
    thread_block_size: usize,
    word_buffer: VecDeque<String>,
    word: String,
    text_stream: Arc<Mutex<S>>,
}

impl<S: Read> AnalyzerThread<S> {
    pub fn new(text_stream: Arc<Mutex<S>>) -> AnalyzerThread<S> {
        Self {
            thread_block_size: 1024,
            word_buffer: VecDeque::new(),
            word: String::new(),
            text_stream,
        }
    }

    pub fn with_block_size(block_size: usize, text_stream: Arc<Mutex<S>>) -> Self {
        let mut thread = Self::new(text_stream);
        thread.thread_block_size = block_size;
        thread
    }

    pub fn analyze(mut self) -> AnalyzerResult {
        let mut result = AnalyzerResult::new();
        let mut has_more = self.get_next_word();

        while has_more {
            let word = self.word.to_lowercase();

            result.total_word_count += 1;
            result.total_letter_count += word.len();

            if word.chars().all(|l| l.is_alphabetic()) {
                if let Some(val) = result.word_heatmap.insert(word.clone(), 1) {
                    result.word_heatmap.insert(word.clone(), val + 1);
                }
            }

            for l in word.chars() {
                if l.is_alphabetic() {
                    if let Some(val) = result.letter_heatmap.insert(l, 1) {
                        result.letter_heatmap.insert(l, val + 1);
                    }
                }
            }

            if result.longest_word.len() < word.len() {
                result.longest_word = word;
            }

            // Get next word
            has_more = self.get_next_word()
        }

        result
    }

    fn get_next_word(&mut self) -> bool {
        let mut has_more = false;

        // If we have a word in our buffer, use it
        if self.word_buffer.len() > 0 {
            if let Some(word) = self.word_buffer.pop_front() {
                self.word = word;
                return true;
            } else {
                panic!("Failed to get next word in own buffer");
            }
        }

        // Otherwise, fill our buffer with more from the stream
        if let Ok(mut stream) = self.text_stream.lock() {
            let mut segment = vec![0; self.thread_block_size];
            let mut read;

            match stream.read(&mut segment) {
                Ok(bytes) => {
                    if let Ok(mut txt) = String::from_utf8(segment) {
                        read = bytes;

                        // This is the end of the stream, last
                        // word will never complete if not
                        // already
                        if read != self.thread_block_size {
                            self.word_buffer
                                .extend(txt.split_whitespace().map(|s| s.to_string()));
                        }
                        // A word that ends without a space may not be complete, therefore we
                        // continue to read until a whitespace is found or the stream is empty
                        else if !txt.ends_with(char::is_whitespace) {
                            let mut bytes: Vec<u8> = Vec::new();
                            let mut byte = [0];
                            loop {
                                match stream.read(&mut byte) {
                                    Ok(b) => {
                                        if b == 0 {
                                            break;
                                        }
                                        bytes.extend_from_slice(&byte);
                                        read += 1;

                                        // If this fails to interpret as utf8 then the character is
                                        // not complete yet
                                        if let Ok(word) = std::str::from_utf8(bytes.as_slice()) {
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
                            self.word_buffer
                                .extend(txt.split_whitespace().map(|s| s.to_string()));
                        }

                        has_more = read > 0;
                    }
                }

                _ => {}
            }
        }

        has_more
    }
}
