use crate::prelude::*;

pub struct MainManager {}
impl MainManager {

    /// Create a new empty instance of MainManager
    pub fn new() -> MainManagerBuilder {
        MainManagerBuilder {build: MainManager {}}
    }

    pub fn run(&mut self) -> Result<()> {

        Ok(())
    }
}

pub struct MainManagerBuilder {
    build: MainManager
}
impl MainManagerBuilder {
    pub fn build(self) -> MainManager {
        self.build
    }
}