#![warn(missing_docs)]
//! Advent of code 2024 Day 2 Challenge
//! 
//! Functions to complete the task for advent of code 2024

use anyhow::{Context, Error, Ok, Result};
use std::fs::File;

/// Reads a file from a given path
/// 
/// Reads a file from a given path and returns String containing full text
/// of the file or Error if the reading was unsuccessfull
/// 
/// # Examples
/// ```
/// use assert_fs::prelude::*;
/// use anyhow::Result;
/// use std::io::Read;
/// 
/// fn main() -> Result<()> {
///     let temp_file = assert_fs::NamedTempFile::new("sample.txt")?;
///     temp_file.write_str("A test\nActual content\nMore content\nAnother test")?;
///
///     let mut file_obj = day_2::read_file(temp_file.path())?;
///     let mut contents = String::new();
///     file_obj.read_to_string(&mut contents)?;
///
///     assert_eq!(contents, "A test\nActual content\nMore content\nAnother test");
///     Ok(())
/// }
/// ```
pub fn read_file(path: &std::path::Path) -> Result<File, Error> {
    let file = File::open(path)
    .with_context(|| format!("could not read file {}", path.display()))?;
    
    Ok(file)
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use assert_fs::prelude::*;
    use anyhow::{Ok, Result};

    #[test]
    fn test_read_file_success() -> Result<()> {
        let temp_file = assert_fs::NamedTempFile::new("sample.txt")?;
        temp_file.write_str("A test\nActual content\nMore content\nAnother test")?;

        let mut file_obj = crate::read_file(temp_file.path())?;
        let mut contents = String::new();
        file_obj.read_to_string(&mut contents)?;

        assert_eq!(contents, "A test\nActual content\nMore content\nAnother test");
        Ok(())
    }

    #[test]
    fn test_read_file_negative() -> Result<()> {
        let result = crate::read_file(std::path::Path::new("sample.txt"));
        assert!(result.is_err_and(|e| e.to_string().contains("could not read file")));
        Ok(())
    }
}