use std::{
    collections::HashMap,
    fmt::Display,
    ops::AddAssign,
    time::{SystemTime, UNIX_EPOCH},
};

use itertools::Itertools;

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
        let top_words: String = self
            .word_heatmap
            .iter()
            .sorted_by(|a, b| b.1.cmp(a.1))
            .take(5)
            .map(|(w, n)| format!("\t{w}: {}\n", int_separate(n)))
            .collect();

        let top_letters: String = self
            .letter_heatmap
            .iter()
            .sorted_by(|a, b| b.1.cmp(a.1))
            .take(5)
            .map(|(l, n)| format!("\t{l}: {}\n", int_separate(n)))
            .collect();

        write!(
            f,
            "\
Source: {}
Scan Time: {}
Total Word Count: {}
Total Letter Count: {}
Longest Word: {}
5 most used words: \n{}
5 most used letters: \n{}",
            self.source_name,
            self.scan_time
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f32(),
            int_separate(&self.total_word_count),
            int_separate(&self.total_letter_count),
            self.longest_word,
            top_words,
            top_letters
        )
    }
}

fn int_separate<N: num::PrimInt + Display>(n: &N) -> String {
    n.to_string()
        .chars()
        .rev()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<String>>()
        .join(",")
        .chars()
        .rev()
        .collect()
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
