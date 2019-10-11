use crate::Error;
use std::path::Path;

pub fn print<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    Ok(())
}
