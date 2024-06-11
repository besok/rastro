#[macro_use]
pub mod iau;

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct UnitError(String);

impl UnitError {
    pub fn add_prefix(&mut self, prefix: String) {
        self.0 = format!("{}: {}", prefix, self.0);
    }
}

impl Display for UnitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration error: {}", self.0)
    }
}