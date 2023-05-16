use std::{fs::OpenOptions, path::PathBuf};

use text_analyzer::analyzer::AnalyzerManager;

mod ui;

fn main() {
    if std::env::args().len() > 1 {
        let mut path = PathBuf::from(std::env::current_dir().unwrap());
        path.push(std::env::args().nth(1).unwrap());

        println!("Opening: {:?}", path);
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("Failed to open file");

        println!("File: {:?}", file);

        let result = AnalyzerManager::new(3, file).analyze();
        println!("Result:\n{}", result);
    } else {
        println!("Usage: {} <file>", std::env::args().nth(0).unwrap());
    }
}
