use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, AddAssign},
    time::{SystemTime, UNIX_EPOCH},
};

/// Structure for keeping track of the analyzed data
pub struct AnalyzerResult {
    pub source_name: String,
    pub scan_time: SystemTime,

    pub total_word_count: usize,
    pub total_letter_count: usize,
    pub longest_word: String,
    pub word_heatmap: HashMap<String, usize>,
    pub letter_heatmap: HashMap<char, usize>,
}

impl AnalyzerResult {
    pub fn new() -> Self {
        AnalyzerResult {
            source_name: String::from(""),
            scan_time: SystemTime::now(),
            total_word_count: 0,
            total_letter_count: 0,
            longest_word: String::new(),
            word_heatmap: HashMap::new(),
            letter_heatmap: HashMap::new(),
        }
    }
}

impl Display for AnalyzerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
            Source: {}
            Scan Time: {}
            Total Word Count: {}
            Total Letter Count: {}
            Longest Word: {}
            Word Heat Map: {:?}
            Letter Heat Map: {:?}
            ",
            self.source_name,
            self.scan_time
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f32(),
            self.total_word_count,
            self.total_letter_count,
            self.longest_word,
            self.word_heatmap,
            self.letter_heatmap
        )
    }
}

impl AddAssign for AnalyzerResult {
    fn add_assign(&mut self, rhs: Self) {
        if self.scan_time < rhs.scan_time {
            self.scan_time = rhs.scan_time
        };

        self.total_word_count += rhs.total_word_count;
        self.total_letter_count += rhs.total_letter_count;

        if self.longest_word.len() < rhs.longest_word.len() {
            self.longest_word = rhs.longest_word
        }

        rhs.word_heatmap.iter().for_each(|(key, val)| {
            if let Some(existing) = self.word_heatmap.insert(key.clone(), *val) {
                self.word_heatmap.insert(key.clone(), existing + *val);
            }
        });

        rhs.letter_heatmap.iter().for_each(|(key, val)| {
            if let Some(existing) = self.letter_heatmap.insert(*key, *val) {
                self.letter_heatmap.insert(*key, existing + val);
            }
        });
    }
}

impl Add for AnalyzerResult {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = AnalyzerResult::new();
        output.source_name = self.source_name;
        output.scan_time = if self.scan_time > rhs.scan_time {
            self.scan_time
        } else {
            rhs.scan_time
        };

        output.total_word_count = self.total_word_count + rhs.total_word_count;
        output.total_letter_count = self.total_letter_count + rhs.total_letter_count;

        output.longest_word = if self.longest_word.len() > rhs.longest_word.len() {
            self.longest_word
        } else {
            rhs.longest_word
        };

        output.word_heatmap = self.word_heatmap.clone();
        for (key, value) in rhs.word_heatmap {
            match self.word_heatmap.get(&key) {
                Some(pre) => {
                    output.word_heatmap.insert(key, *pre + value);
                }
                None => {
                    output.word_heatmap.insert(key, value);
                }
            }
        }

        output.letter_heatmap = self.letter_heatmap.clone();
        for (key, value) in rhs.letter_heatmap {
            match self.letter_heatmap.get(&key) {
                Some(pre) => {
                    output.letter_heatmap.insert(key, *pre + value);
                }
                None => {
                    output.letter_heatmap.insert(key, value);
                }
            }
        }

        output
    }
}
