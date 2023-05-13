use std::collections::{HashMap, HashSet};
use std::iter::Map;
use std::vec::IntoIter;

pub struct SourceManager {
    source_list: HashMap<String, Box<dyn SourceLoader>>
}
impl SourceManager {
    pub fn get_text(source: &str) -> IntoIter<String> {
        if let source = source.split(":").next()
        else {
            unimplemented!();
        };
    }
}


trait SourceLoader {
    fn get_text_stream(path: string) -> IntoIter<String>;
}