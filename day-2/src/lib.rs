#![warn(missing_docs)]
//! Advent of code 2024 Day 2 Challenge
//! 
//! Functions to complete the task for advent of code 2024

use anyhow::{Context, Error, Ok, Result};

/// Reads a file from a given path
/// 
/// Reads a file from a given path and returns String containing full text
/// of the file or Error if the reading was unsuccessfull
/// 
/// # Examples
/// ```
/// use assert_fs::prelude::*;
/// use anyhow::Result;
/// 
/// fn main() -> Result<()> {
///     let file = assert_fs::NamedTempFile::new("sample.txt")?;
///     file.write_str("Example text")?;
///     let result = day_2::read_file(file.path())?;
///     println!("{}", result);
///     Ok(())
/// }
/// ```
pub fn read_file(path: &std::path::Path) -> Result<String, Error> {
    let contents = std::fs::read_to_string(path)
    .with_context(|| format!("could not read file {}", path.display()))?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::*;
    use anyhow::{Ok, Result};

    #[test]
    fn test_read_file_success() -> Result<()> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("A test\nActual content\nMore content\nAnother test")?;

        let result = crate::read_file(file.path())?;
        assert_eq!(result, "A test\nActual content\nMore content\nAnother test");
        Ok(())
    }

    #[test]
    fn test_read_file_negative() -> Result<()> {
        let result = crate::read_file(std::path::Path::new("sample.txt"));
        assert!(result.is_err_and(|e| e.to_string().contains("could not read file")));
        Ok(())
    }
}