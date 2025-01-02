#![warn(missing_docs)]
//! Advent of code 2024 Day 4 Challenge
//! 
//! Functions to complete the task for advent of code 2024
//! 
//! [`Read more`](../../../README.md)

use std::{fs::File, io::{BufRead, BufReader, Lines}};
use anyhow::{Context, Error, Ok};

/// Form a word from a string vector
/// 
/// Form a 3 letter word in a required direction
/// 
/// # Parameters
/// 
/// * `lines_vec` - A reference to a vector of strings. At least 3
/// lines are requered to form 3 letter vertical / diagonal words.
/// * `idx` - An index of the starting symbol in a line / string.
/// * `dir` - Direction to form word `0` (vertical), `-1` (left diagonal), `1` (right diagonal).
/// 
/// # Returns
/// 
/// * `String` - Formed word in a chosen direction.
fn form_word(lines_vec: &Vec<String>, idx: usize, dir: i32) -> String {
    let mut s = String::new();
    if ![0, -1, 1].contains(&dir) {
        return s;
    }
    if lines_vec.len() < 3 {
        return s;
    }
    for i in 0..3 {
        let mut chars = lines_vec[(2 - i) as usize].chars();
        let new_idx = idx as isize + i as isize * dir as isize;
        if new_idx < 0 {
            return s;
        }
        let opt = chars.nth(new_idx as usize);
        match opt {
            Some(c) => s.push(c),
            None => return s
        }
    }
    return s;
}

/// Count the amount of vertical and diagonal words in a string vector
/// 
/// # Parameters
/// 
/// * `lines_vec` - A reference to a vector of strings. At least 3
/// lines are requered to form 3 letter vertical / diagonal words.
/// 
/// # Returns
/// 
/// * `Result<usize, Error>` - Number of `MAS` in provided vector.
fn count_verticals(lines_vec: &Vec<String>) -> Result<usize, Error> {
    let mut count = 0;
    if lines_vec.len() < 3 {
        return Ok(count);
    }
    for i in 0..lines_vec[2].len() {
        for j in [-1, 1] {
            let s = form_word(lines_vec, i, j);
            count += s.matches("MAS").count();
            count += s.matches("SAM").count();
        }
    }
    Ok(count)
}

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
    let opt = iter.next();
    match opt {
        Some(new) => v[2] = new.with_context(|| "failed reading line")?,
        None => v[2] = String::new()
    }
    Ok(v.to_vec())

}

/// Initialize a 3-line vector from a [BufReader] iterator.
/// 
/// Reads up to 3 lines from the provided iterator and initializes a vector with these lines.
/// If fewer than 3 lines are available, the remaining entries in the vector are filled with empty strings.
/// 
/// # Parameters
/// 
/// * `lines_iter` - A mutable reference to an iterator over lines in a buffered reader.
/// 
/// # Returns
/// 
/// * `Result<Vec<String>, Error>` - A vector containing up to 3 lines read from the iterator, or an error.
fn vec_init<B: BufRead>(lines_iter: &mut Lines<B>) -> Result<Vec<String>, Error> {
    let mut v = Vec::new();
    for _ in 0..3 {
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
///     let c = day_4::xmas_count(std::path::Path::new("./data/input_test_9.txt"))?;
///     assert_eq!(c, 25);
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
        // count vertical and diagonal matches
        count += count_verticals(&lines_vec)?;

        // update lines vector
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
    fn test_count_verticals() -> Result<()> {
        let data = vec![
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string()];
        let count = crate::count_verticals(&data)?;
        assert_eq!(count, 4);
        Ok(())
    }

    #[test]
    fn test_form_word_right_diag() {
        let data = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string()];
        let s = form_word(&data, 5, 1);
        assert_eq!(s, "MSA");
    }

    #[test]
    fn test_form_word_left_diag() {
        let data = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string()];
        let s = form_word(&data, 5, -1);
        assert_eq!(s, "MXS");
    }
    
    #[test]
    fn test_form_word_vertical() {
        let data = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string()];
        let s = form_word(&data, 5, 0);
        assert_eq!(s, "MMX");
    }

    #[test]
    fn test_xmas_count() -> Result<()> {
        let c = xmas_count(std::path::Path::new("./data/input_test_9.txt"))?;
        assert_eq!(c, 25);
        Ok(())
    }

    #[test]
    fn test_vec_update() -> Result<()> {
        let data = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM";
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        
        let mut lines_iter = reader.lines();
        lines_iter.nth(2); // Skip the first 2 lines
    
        let mut lines_vec = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string()
        ];
    
        lines_vec = vec_update(&mut lines_vec, &mut lines_iter)?;
    
        assert_eq!(lines_vec, vec![
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string()
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
        "AMXSXMAAMM".to_string()]);
        Ok(())
    }
}