#![warn(missing_docs)]
//! Advent of code 2024 Day 3 Challenge
//! 
//! Functions to complete the task for advent of code 2024
//! 
//! [`Read more`](../../../README.md)

use anyhow::{Context, Error, Ok, Result};
use regex::Regex;
use std::{fs::File, io::{BufRead, BufReader}};

/// Filter the corrupted memory
/// 
/// Filter corrupted memory and return only uncorrupted instructions
/// 
/// # Examples
/// ```
/// use anyhow::Result;
/// 
/// fn main() -> Result<()> {
///     let file = day_3::read_file(std::path::Path::new("./data/input_test_161.txt"))?;
///     let v = day_3::filter_corrupted(file)?;
///     assert_eq!(v, vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]);
///     Ok(())
/// }
/// ```
pub fn filter_corrupted(file: File) -> Result<Vec<String>, anyhow::Error> {
    let mut filtered = Vec::new();
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let contents:String = line.with_context(|| format!("failed reading line"))?;
        let re = Regex::new(r"mul\([\d]{1,3},[\d]{1,3}\)").unwrap();
        let mut uncorrupted = re.find_iter(&contents)
        .map(|m| m.as_str().to_string()).collect();
        filtered.append(&mut uncorrupted);
    }

    Ok(filtered)
}

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
///     let mut file_obj = day_3::read_file(temp_file.path())?;
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
    fn test_filter_corrupted_success() -> Result<()> {
        let file = crate::read_file(std::path::Path::new("./data/input_test_161.txt"))?;
        let v = crate::filter_corrupted(file)?;
        assert_eq!(v, vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]);
        Ok(())
    }

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
