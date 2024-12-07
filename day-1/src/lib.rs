#![warn(missing_docs)]
//! Advent of code 2024 Day 1 Challenge
//! 
//! Functions to complete the task for advent of code 2024

use anyhow::{Context, Error, Ok, Result};


/// Find distance between 2 vectors
/// 
/// Compare each number in 2 vectors and
/// accumulate total distance
/// 
/// # Example
/// ```
/// use anyhow::Result;
/// 
/// fn main() -> Result<()> {
///     let v1 = vec!["3", "4", "2", "1", "3", "3"];
///     let v2 = vec!["4", "3", "5", "3", "9", "3"];
///     let dist = day_1::distance(v1, v2)?;
///     assert_eq!(dist, 11);
///     Ok(())
/// }
/// ```
pub fn distance(list1: Vec<&str>, list2: Vec<&str>) -> Result<i32> {
    let mut dist: i32 = 0;

    let mut sorted_list1 = list1.clone();
    sorted_list1.sort();
    let mut sorted_list2 = list2.clone();
    sorted_list2.sort();

    for i in 0..sorted_list1.len() {
        let sym1 = sorted_list1.get(i)
        .with_context(|| format!("failed reading symbol from List 1"))?;
        let num1 = sym1.parse::<i32>()
        .with_context(|| format!("failed parsing {} to number", sym1))?;
        let sym2 = sorted_list2.get(i)
        .with_context(|| format!("failed reading symbol from List 2"))?;
        let num2 = sym2.parse::<i32>()
        .with_context(|| format!("failed parsing {} to number", sym2))?;

        let dist_i = num1 - num2;
        dist = dist + dist_i.abs();
    }

    Ok(dist)
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
/// 
/// fn main() -> Result<()> {
///     let file = assert_fs::NamedTempFile::new("sample.txt")?;
///     file.write_str("Example text")?;
///     let result = day_1::read_file(file.path())?;
///     println!("{}", result);
///     Ok(())
/// }
/// ```
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
    use crate::{distance, get_lists, read_file};

    #[test]
    fn test_distance_success() -> Result<()> {
        let v1 = vec!["3", "4", "2", "1", "3", "3"];
        let v2 = vec!["4", "3", "5", "3", "9", "3"];
        let dist = distance(v1, v2)?;
        assert_eq!(dist, 11);
        Ok(())
    }

    #[test]
    fn test_distance_negative() -> Result<()> {
        let v1 = vec!["3", "4", "asd", "1", "3", "3"];
        let v2 = vec!["4", "3", "5", "3", "9", "3"];
        let dist = distance(v1, v2);
        assert!(dist.is_err_and(|e| e.to_string().eq("failed parsing asd to number")));
        Ok(())
    }

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