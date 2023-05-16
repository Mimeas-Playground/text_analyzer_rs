use super::AnalyzerResult;
use std::{
    collections::vec_deque::VecDeque,
    io::Read,
    sync::{Arc, Mutex},
};

pub struct AnalyzerThread<S: Read> {
    text: VecDeque<String>,
    word: String,
    text_stream: Arc<Mutex<S>>,
}

impl<S: Read> AnalyzerThread<S> {
    pub fn new(text_stream: Arc<Mutex<S>>) -> AnalyzerThread<S> {
        Self {
            text: VecDeque::new(),
            word: String::new(),
            text_stream,
        }
    }

    pub fn analyze(mut self) -> AnalyzerResult {
        let mut result = AnalyzerResult::new();
        let mut has_more = self.get_next_word();

        while has_more {
            result.total_word_count += 1;
            result.total_letter_count += self.word.len();

            if let Some(val) = result.word_heatmap.insert(self.word.clone(), 1) {
                result.word_heatmap.insert(self.word.clone(), val + 1);
            }

            for l in self.word.chars() {
                if let Some(val) = result.letter_heatmap.insert(l, 1) {
                    result.letter_heatmap.insert(l, val + 1);
                }
            }

            // Get next word
            has_more = self.get_next_word()
        }

        result
    }

    fn get_next_word(&mut self) -> bool {
        let mut has_more = false;
        if self.text.len() > 0 {
            if let Some(word) = self.text.pop_front() {
                self.word = word;
                return true;
            } else {
                panic!("Failed to get next word in own buffer");
            }
        }

        if let Ok(mut stream) = self.text_stream.lock() {
            println!("Aquiring more text");
            let mut segment = Vec::with_capacity(1024);
            let mut read;

            match stream.read(segment.as_mut_slice()) {
                Ok(bytes) => {
                    if let Ok(txt) = String::from_utf8(segment) {
                        read = bytes;
                        txt.split(char::is_whitespace)
                            .for_each(|w| self.text.push_back(w.to_string()));

                        if !self.text.back().unwrap().ends_with(char::is_whitespace) {
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
                                        if let Ok(word) = std::str::from_utf8(bytes.as_slice()) {
                                            if word.ends_with(char::is_whitespace) {
                                                break;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        if e.kind() == std::io::ErrorKind::WouldBlock {
                                            break;
                                        } else {
                                            panic!("Error reading from stream: {}", e);
                                        }
                                    }
                                }
                            }

                            if let Ok(word) = std::str::from_utf8(bytes.as_slice()) {
                                self.text.back_mut().unwrap().push_str(word);
                            }
                        }

                        if read > 0 {
                            has_more = true;
                        }
                    }
                }

                _ => {}
            }
        }

        return has_more;
    }
}
