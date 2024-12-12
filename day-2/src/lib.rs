#![warn(missing_docs)]
//! Advent of code 2024 Day 2 Challenge
//! 
//! Functions to complete the task for advent of code 2024
//! 
//! [`Read more`](../../../README.md)

use anyhow::{Context, Error, Ok, Result};
use std::{fs::File, io::{BufRead, BufReader}};

/// Find the number of safe reports
/// 
/// Returns a number of safe reports or error of operation failed
/// Uses [`read_file`] to read file and 
/// [`safe_report`] to determine if report is safe
/// 
/// # Example
/// ```
/// use anyhow::Result;
/// 
/// fn main() -> Result<()> {
///     let file = day_2::read_file(std::path::Path::new("./data/input_test_2.txt"))?;
///
///     let num_safe_reports = day_2::safe_reports_number(file)?;
///
///     assert_eq!(num_safe_reports, 2);
///     Ok(())
/// }
/// ```
pub fn safe_reports_number(file: File) -> Result<i32, Error> {
    let mut num_safe_reports = 0;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let report = line.with_context(|| format!("failed to read line"))?;
        let report_safe = crate::safe_report(&report)?;
        if report_safe {
            num_safe_reports += 1;
        }
    }

    Ok(num_safe_reports)
}

/// Check if 2 number sequence is descending
/// 
/// If numbers are not descending return false
/// 
/// # Examples
/// ```
/// use anyhow::Result;
/// 
/// fn main() -> Result<()> {
///     let is_descending = day_2::is_descending(3, 1);
///     assert!(is_descending);
///     Ok(())
/// }
/// ```
pub fn is_descending(previous_number: i32, current_number: i32) -> bool {
    if current_number == previous_number {
        return false;
    }
    if current_number > previous_number {
        return false;
    }
    if current_number < previous_number {
        let diff = previous_number - current_number;
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    return true;
}

/// Check if 2 number sequence is ascending
/// 
/// If numbers are not ascending return false
/// 
/// # Examples
/// ```
/// use anyhow::Result;
/// 
/// fn main() -> Result<()> {
///     let is_ascending = day_2::is_ascending(1, 2);
///     assert!(is_ascending);
///     Ok(())
/// }
/// ```
pub fn is_ascending(previous_number: i32, current_number: i32) -> bool {
    if current_number == previous_number {
        return false;
    }
    if current_number < previous_number {
        return false;
    }
    if current_number > previous_number {
        let diff = current_number - previous_number;
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    return true;
}

/// Determines whether provided report is safe or not
/// 
/// Returns `true` if the provided report is safe, returns `false` otherwise
/// Uses functions [`is_ascending`] and [`is_descending`] to
/// determine if the reports are safe
/// 
/// # Examples
/// ```
/// use anyhow::Result;
/// 
/// fn test_inspect_report_success_safe() -> Result<()> {
///     let rep = "1 2 3 5 8";
///     let safe = day_2::safe_report(rep)?;
///     assert!(safe);
///     Ok(())
/// }
/// ```
pub fn safe_report(report: &str) -> Result<bool, Error> {
    let symbols: Vec<&str> = report.split(" ").collect();
    let mut prev_is_ascending = false;
    let mut prev_is_descending = false;
    for i in 1..symbols.len() {
        let sym_curr = symbols.get(i)
        .with_context(|| format!("failed getting a symbol with index {}", i))?;
        let num_curr = sym_curr.parse::<i32>()
        .with_context(|| format!("failed parsing {} to number", sym_curr))?;

        let sym_prev = symbols.get(i-1)
        .with_context(|| format!("failed getting a symbol with index {}", i-1))?;
        let num_prev = sym_prev.parse::<i32>()
        .with_context(|| format!("failed parsing {} to number", sym_prev))?;

        let is_ascending = crate::is_ascending(num_prev, num_curr);
        let is_descending = crate::is_descending(num_prev, num_curr);

        if !is_ascending && !is_descending {
            return Ok(false);
        }
        if i > 1 {
            if is_ascending && !prev_is_ascending {
                return Ok(false);
            }
            if is_descending && !prev_is_descending {
                return Ok(false);
            }
        }
        prev_is_ascending = is_ascending;
        prev_is_descending = is_descending;
    }
    Ok(true)
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
    fn test_safe_reports_number() -> Result<()> {
        let file = crate::read_file(std::path::Path::new("./data/input_test_2.txt"))?;

        let num_safe_reports = crate::safe_reports_number(file)?;

        assert_eq!(num_safe_reports, 2);
        Ok(())
    }

    #[test]
    fn test_inspect_report_success_safe() -> Result<()> {
        let rep = "1 2 3 5 8";
        let safe = crate::safe_report(rep)?;
        assert!(safe);
        Ok(())
    }

    #[test]
    fn test_inspect_report_success_unsafe() -> Result<()> {
        let rep = "2 1 2 2 1 4";
        let not_safe = !crate::safe_report(rep)?;
        assert!(not_safe);
        Ok(())
    }

    #[test]
    fn test_inspect_report_negative() -> Result<()> {
        let rep = "1 2 4 asd 5 8";
        let safe = crate::safe_report(rep);
        assert!(safe.is_err_and(|e| e.to_string().eq("failed parsing asd to number")));
        Ok(())
    }

    #[test]
    fn test_is_descending_succes() -> Result<()> {
        let is_descending = crate::is_descending(3, 1);
        assert!(is_descending);
        Ok(())
    }

    #[test]
    fn test_is_ascending_succes() -> Result<()> {
        let is_ascending = crate::is_ascending(1, 2);
        assert!(is_ascending);
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