use std::{
    env,
    fs::{self, OpenOptions},
    io::BufReader,
};

use text_analyzer::analyzer::AnalyzerManager;

mod ui;

fn main() {
    if std::env::args().len() > 1 {
        let file = OpenOptions::new()
            .open(env::args().nth(1).unwrap())
            .expect("Failed to open file");

        println!("File: {:?}", file);

        let result = AnalyzerManager::new(3, BufReader::new(file)).analyze();
        println!("Result:\n{}", result);
    } else {
        println!("Usage: {} <file>", std::env::args().nth(0).unwrap());
    }
}
