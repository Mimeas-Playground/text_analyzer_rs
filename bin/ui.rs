pub fn main_menu() {
    println!("1. Analyze Texts");
    println!("2. Get text stats");
    println!("3. Write your own text");
    println!("4 Exit");
}

pub fn save_or_discard() {
    println!("1. Save data");
    println!("2. Discard data");
}

pub fn ProgressBar() {
    println!("Analyzing Text...");
    // TODO Instead of using the artificial progress bar from the reference, use a real progress bar
    // with a thread wich checks the file's size and the current position of the cursor at regular
    // intervals
}

pub fn PrintBackToMainMenu() {
    println!("1. Back to main menu");
}
