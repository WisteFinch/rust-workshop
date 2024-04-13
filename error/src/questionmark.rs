use std::error::Error;

// Try using Anyhow
pub fn questionmark(foreign_error: Result<(), impl Error + std::marker::Send + std::marker::Sync + 'static>) -> anyhow::Result<()> {
    foreign_error?;
    Ok(())
}
