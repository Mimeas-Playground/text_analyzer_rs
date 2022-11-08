
pub use crate::error::Error;
pub type Result<T> = core::result::Result<T, Error>;
pub use crate::main_manager::{MainManager, MainManagerBuilder};