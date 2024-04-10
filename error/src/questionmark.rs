use std::error::Error;

// Try using Anyhow
pub fn questionmark(foreign_error: Result<(), impl Error>) -> anyhow::Result<()> {
    todo!()
}
