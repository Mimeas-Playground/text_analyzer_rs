use crate::error::Error::Build;
use crate::prelude::*;

pub struct MainManager {
    source: SourceManager
}
impl MainManager {

    /// Create a new empty instance of MainManager
    pub fn new() -> MainManagerBuilder {
        MainManagerBuilder {source: None}
    }

    pub fn run(&mut self) -> Result<()> {

        Ok(())
    }
}

pub struct MainManagerBuilder {
    source: Option<SourceManager>
}
impl MainManagerBuilder {

    pub fn build(self) -> Result<MainManager> {
        if self.source.is_none() {
            Err(Build("Missing source component".to_string()))
        }
        else {
            Ok(
                MainManager {
                    source: self.source
                }
            )
        }
    }

    pub fn with_source_manager(mut self, source: SourceManager) -> MainManagerBuilder {
        self.source = source;
        self
    }
}