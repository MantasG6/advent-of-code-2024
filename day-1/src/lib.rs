#![warn(missing_docs)]
//! Advent of code 2024 Day 1 Challenge
//! 
//! Functions to complete the task for advent of code 2024
//! 

use anyhow::{Context, Error, Result};

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
///     let result = day_1::read_file(file.path())?;
///     println!("{}", result);
///     Ok(())
/// }
/// ```
/// 
pub fn read_file(path: &std::path::Path) -> Result<String, Error> {
    let contents = std::fs::read_to_string(path)
    .with_context(|| format!("could not read file {}", path.display()))?;

    Ok(contents)
}

/// Creates the lists of numbers
/// 
/// Based on provided text (String) splits the numbers
/// and puts them in 2 separate lists
/// 
/// #Examples
/// ```
/// use anyhow::{Ok, Result};
/// 
/// fn main() -> Result<()> {
///     let text = "123   45\n33   45\n2   45\n";
///     let (l1,l2) = day_1::get_lists(&text)?;
///     assert!(!l1.is_empty());
///     assert!(!l2.is_empty());
///     Ok(())
/// }
/// ```
pub fn get_lists(text: &str) -> Result<(Vec<&str>, Vec<&str>), Error> {
    let mut list1: Vec<&str> = Vec::new();
    let mut list2: Vec<&str> = Vec::new();
    for line in text.lines() {
        let mut numbers = line.split("   ");

        let number1 = numbers.next()
        .with_context(|| format!("split failed"))?;
        
        let number2 = numbers.next()
        .with_context(|| format!("split failed"))?;

        list1.push(number1);
        list2.push(number2);
    }

    Ok((list1, list2))
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::*;
    use anyhow::{Ok, Result};
    use crate::{get_lists, read_file};


    #[test]
    fn test_read_file_success() -> Result<()> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("A test\nActual content\nMore content\nAnother test")?;

        let result = read_file(file.path())?;
        assert_eq!(result, "A test\nActual content\nMore content\nAnother test");
        Ok(())
    }

    #[test]
    fn test_read_file_negative() -> Result<()> {
        let result = read_file(std::path::Path::new("sample.txt"));
        assert!(result.is_err_and(|e| e.to_string().contains("could not read file")));
        Ok(())
    }

    #[test]
    fn test_get_lists_success() -> Result<()> {
        let text = "123   45\n33   45\n2   45\n";
        let (l1,l2) = get_lists(&text)?;
        assert!(!l1.is_empty());
        assert!(!l2.is_empty());
        Ok(())
    }

    #[test]
    fn test_get_lists_negative() -> Result<()> {
        let text = "123";
        let result = get_lists(&text);
        assert!(result.is_err_and(|e| e.to_string().contains("split failed")));
        Ok(())
    }
}