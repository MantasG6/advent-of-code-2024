#![warn(missing_docs)]
//! Advent of code 2024 Day 4 Challenge
//! 
//! Functions to complete the task for advent of code 2024
//! 
//! [`Read more`](../../../README.md)

use std::{fs::File, io::{BufRead, BufReader, Lines}};
use anyhow::{Context, Error, Ok};

/// Shift vector by 1 iteration
/// 
/// Shift vector by 1 line, removing the first line and
/// adding a new line from the iterator to the end of the vector.
/// Adds an empty string if the end of the iterator is reached.
/// 
/// # Parameters
/// 
/// * `v` - A mutable reference to a vector of strings.
/// * `iter` - A mutable reference to an iterator over lines in a buffered reader.
/// 
/// # Returns
/// 
/// * `Result<Vec<String>, Error>` - The updated vector or an error.
fn vec_update<B: BufRead>(v: &mut Vec<String>, iter: &mut Lines<B>) -> Result<Vec<String>, Error> {
    v[0] = v[1].clone();
    v[1] = v[2].clone();
    v[2] = v[3].clone();
    let opt = iter.next();
    match opt {
        Some(new) => v[3] = new.with_context(|| "failed reading line")?,
        None => v[3] = String::new()
    }
    Ok(v.to_vec())

}

/// Initialize a 4-line vector from a [BufReader] iterator.
/// 
/// Reads up to 4 lines from the provided iterator and initializes a vector with these lines.
/// If fewer than 4 lines are available, the remaining entries in the vector are filled with empty strings.
/// 
/// # Parameters
/// 
/// * `lines_iter` - A mutable reference to an iterator over lines in a buffered reader.
/// 
/// # Returns
/// 
/// * `Result<Vec<String>, Error>` - A vector containing up to 4 lines read from the iterator, or an error.
fn vec_init<B: BufRead>(lines_iter: &mut Lines<B>) -> Result<Vec<String>, Error> {
    let mut v = Vec::new();
    for _ in 0..4 {
        let opt = lines_iter.next();
        match opt {
            Some(res) => v.push(res.with_context(|| "failed reading line")?),
            None => v.push(String::new())
        }
    }
    Ok(v)
}

/// Count XMAS matches in a file
/// 
/// Count horizontal, vertical, diagonal, backwards and overflowing XMAS matches
/// in a provided file path
/// 
/// # Parameters
/// 
/// * `input_path` - A reference to the path of the input file.
/// 
/// # Returns
/// 
/// * `Result<usize, Error>` - The count of XMAS matches or an error.
/// 
/// # Examples
/// ```
/// use anyhow::Result;
/// 
/// fn main() -> Result<()> {
///     let c = day_4::xmas_count(std::path::Path::new("./data/input_test_18.txt"))?;
///     assert_eq!(c, 5);
///     Ok(())
/// }
/// ```
pub fn xmas_count(input_path: &std::path::Path) -> Result<usize, Error> {
    let mut count = 0;

    let file = File::open(input_path)
    .with_context(|| format!("failed to open file {}", input_path.display()))?;
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines();

    let mut lines_vec = vec_init(&mut lines_iter)?;

    while !lines_vec[0].is_empty() {
        // count horizontal matches
        count += lines_vec[0].matches("XMAS").count();
        // count backwards matches
        count += lines_vec[0].matches("SAMX").count();
        lines_vec = vec_update(&mut lines_vec, &mut lines_iter)?;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Cursor};
    use anyhow::{Ok, Result};

    #[test]
    fn test_xmas_count() -> Result<()> {
        let c = xmas_count(std::path::Path::new("./data/input_test_18.txt"))?;
        assert_eq!(c, 5);
        Ok(())
    }

    #[test]
    fn test_vec_update() -> Result<()> {
        let data = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM";
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        
        let mut lines_iter = reader.lines();
        lines_iter.nth(3); // Skip the first 4 lines
    
        let mut lines_vec = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string()
        ];
    
        lines_vec = vec_update(&mut lines_vec, &mut lines_iter)?;
    
        assert_eq!(lines_vec, vec![
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string()
        ]);
        Ok(())
    }

    #[test]
    fn test_vec_init() -> Result<()> {
        let data = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nMSAMASMSMX";
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
    
        let mut lines_iter = reader.lines();

        let lines_vec = crate::vec_init(&mut lines_iter)?;

        assert_eq!(lines_vec, vec![
        "MMMSXXMASM".to_string(),
        "MSAMXMSMSA".to_string(),
        "AMXSXMAAMM".to_string(),
        "MSAMASMSMX".to_string()]);
        Ok(())
    }
}