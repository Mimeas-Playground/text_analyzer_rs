use super::AnalyzerResult;
use std::{
    collections::vec_deque::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
};

pub struct AnalyzerThread {
    text: VecDeque<String>,
    word: String,
    text_stream: Arc<Mutex<BufReader<File>>>,
}

impl AnalyzerThread {
    pub fn new(text_stream: Arc<Mutex<BufReader<File>>>) -> AnalyzerThread {
        Self {
            text: VecDeque::new(),
            word: String::new(),
            text_stream,
        }
    }

    pub fn analyze(mut self) -> AnalyzerResult {
        let mut has_more = false;
        let mut result = AnalyzerResult::new();

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
                has_more = true;
            }
        } else {
            if let Ok(stream) = self.text_stream.lock() {
                if stream.buffer().is_empty() {
                    has_more = false
                } else {
                    let mut line = String::new();
                    if let Ok(_) = stream.buffer().read_line(&mut line) {
                        line.split(' ')
                            .for_each(|w| self.text.push_back(w.to_string()));
                        has_more = true
                    }
                }
            }
        }

        return has_more;
    }
}
