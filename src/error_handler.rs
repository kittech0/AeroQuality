use std::error::Error;
use std::fmt::{Display, Formatter};

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct GtkError(pub i32);

impl Display for GtkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error code: {}. More information above.", self.0)
    }
}

impl Error for GtkError {}
