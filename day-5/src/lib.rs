#![warn(missing_docs)]
//! Advent of code 2024 Day 5 Challenge
//! 
//! Functions to complete the task for advent of code 2024
//! 
//! [`Read more`](../../../README.md)

use std::{collections::HashMap, io::BufRead};
use anyhow::{Context, Error, Result};

/// Read rules and add them to a hash map
/// 
/// From each rule first number (lower one) is mapped
/// to all the numbers that are greater
/// The map keys are the numbers provided in the rules.
/// The map values are lists of numbers greater (in order) than the key.
/// 
/// # Examples
/// ```
/// use std::io::{BufReader, Cursor};
/// use anyhow::{Ok, Result};
/// use std::collections::HashMap;
/// 
/// fn main() -> Result<()> {
///     let data = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n\
///     53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13";
///     let cursor = Cursor::new(data);
///     let mut reader = BufReader::new(cursor);
/// 
///     let real_map = day_5::read_rules(&mut reader)?;
///     let test_map = HashMap::from([
///         ("29".to_string(), vec!["13".to_string()]),
///         ("53".to_string(), vec!["29".to_string(),
///         "13".to_string()]),
///         ("61".to_string(), vec!["13".to_string(),
///         "53".to_string(),
///         "29".to_string()]),
///         ("47".to_string(), vec!["53".to_string(),
///         "13".to_string(),
///         "61".to_string(),
///         "29".to_string()]),
///         ("75".to_string(), vec!["29".to_string(),
///         "53".to_string(),
///         "47".to_string(),
///         "61".to_string(),
///         "13".to_string()]),
///         ("97".to_string(), vec!["13".to_string(),
///         "61".to_string(),
///         "47".to_string(),
///         "29".to_string(),
///         "53".to_string(),
///         "75".to_string()])
///     ]);
/// 
///     assert_eq!(test_map, real_map);
/// 
///     Ok(())
/// }
/// ```
pub fn read_rules<B: BufRead>(reader: &mut B) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let l = line.with_context(|| "failed reading line")?;
        let opt = l.split_once("|");
        let key;
        let val;
        match opt {
            Some((lower, greater)) => {
                key = lower;
                val = greater;
            },
            None => return Ok(map)
        }
        if map.contains_key(key) {
            map.entry(key.to_string()).and_modify(|vec| vec.push(val.to_string()));
        } 
        else {
            map.insert(key.to_string(), vec![val.to_string()]);
        }
    }
    Ok(map)
}

/// Determine if pages are correctly ordered
/// 
/// Read the pages and determine if they are ordered
/// according to the provided rules
fn is_correctly_ordered(rules: &HashMap<String, Vec<String>>, update_vec: &Vec<&str>) -> bool {
    for i in 0..update_vec.len()-1 {
        let curr = update_vec.get(i).unwrap_or(&"").to_string();
        let curr_greater;
        match rules.get(&curr) {
            Some(val) => curr_greater = val,
            None => return false
        }
        for j in i+1..update_vec.len() {
            let next = update_vec.get(j).unwrap_or(&"").to_string();
            if !curr_greater.contains(&next) {
                return false;
            }
        }
    }
    true
}

/// Sum middle page numbers of correctly ordered updates
/// 
/// Find all the correctly ordered updates and add
/// all middle page numbers to get the sum
/// 
/// # Examples
/// ```
/// use std::io::{BufReader, Cursor};
/// use anyhow::{Ok, Result};
/// use std::collections::HashMap;
/// 
/// fn main() -> Result<()> {
///     let data = "75,47,61,53,29\n97,61,53,29,13\n\
///     75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
/// 
///     let rules = HashMap::from([
///         ("29".to_string(), vec!["13".to_string()]),
///         ("53".to_string(), vec!["29".to_string(),
///         "13".to_string()]),
///         ("61".to_string(), vec!["13".to_string(),
///         "53".to_string(),
///         "29".to_string()]),
///         ("47".to_string(), vec!["53".to_string(),
///         "13".to_string(),
///         "61".to_string(),
///         "29".to_string()]),
///         ("75".to_string(), vec!["29".to_string(),
///         "53".to_string(),
///         "47".to_string(),
///         "61".to_string(),
///         "13".to_string()]),
///         ("97".to_string(), vec!["13".to_string(),
///         "61".to_string(),
///         "47".to_string(),
///         "29".to_string(),
///         "53".to_string(),
///         "75".to_string()])
///     ]);
/// 
///     let cursor = Cursor::new(data);
///     let mut reader = BufReader::new(cursor);
/// 
///     let sum = day_5::correctly_ordered_sum(&mut reader, &rules)?;
/// 
///     assert_eq!(sum, 143);
/// 
///     Ok(())
/// }
/// ```
pub fn correctly_ordered_sum<B: BufRead>(reader: &mut B, 
    rules: &HashMap<String, Vec<String>>) -> Result<i32, Error> {
    let mut sum = 0;

    for line in reader.lines() {
        let update = line.with_context(|| "failed to read line")?;
        let update_vec = update.split(",").collect();
        if !is_correctly_ordered(rules, &update_vec) {
            continue;
        }
        sum += update_vec.get(update_vec.len()/2)
        .unwrap_or(&"")
        .parse::<i32>()
        .unwrap_or(0);
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};
    use anyhow::{Ok, Result};

    #[test]
    fn test_sum() -> Result<()> {
        let data = "75,47,61,53,29\n97,61,53,29,13\n\
        75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

        let rules = HashMap::from([
            ("29".to_string(), vec!["13".to_string()]),
            ("53".to_string(), vec!["29".to_string(),
            "13".to_string()]),
            ("61".to_string(), vec!["13".to_string(),
            "53".to_string(),
            "29".to_string()]),
            ("47".to_string(), vec!["53".to_string(),
            "13".to_string(),
            "61".to_string(),
            "29".to_string()]),
            ("75".to_string(), vec!["29".to_string(),
            "53".to_string(),
            "47".to_string(),
            "61".to_string(),
            "13".to_string()]),
            ("97".to_string(), vec!["13".to_string(),
            "61".to_string(),
            "47".to_string(),
            "29".to_string(),
            "53".to_string(),
            "75".to_string()])
        ]);

        let cursor = Cursor::new(data);
        let mut reader = BufReader::new(cursor);

        let sum = correctly_ordered_sum(&mut reader, &rules)?;

        assert_eq!(sum, 143);

        Ok(())
    }

    #[test]
    fn test_not_correctly_ordered() {
        let rules = HashMap::from([
            ("29".to_string(), vec!["13".to_string()]),
            ("53".to_string(), vec!["29".to_string(),
            "13".to_string()]),
            ("61".to_string(), vec!["13".to_string(),
            "53".to_string(),
            "29".to_string()]),
            ("47".to_string(), vec!["53".to_string(),
            "13".to_string(),
            "61".to_string(),
            "29".to_string()]),
            ("75".to_string(), vec!["29".to_string(),
            "53".to_string(),
            "47".to_string(),
            "61".to_string(),
            "13".to_string()]),
            ("97".to_string(), vec!["13".to_string(),
            "61".to_string(),
            "47".to_string(),
            "29".to_string(),
            "53".to_string(),
            "75".to_string()])
        ]);
        let update = vec!["75","97","47","61","53"];

        assert!(!is_correctly_ordered(&rules, &update));
    }

    #[test]
    fn test_correctly_ordered() {
        let rules = HashMap::from([
            ("29".to_string(), vec!["13".to_string()]),
            ("53".to_string(), vec!["29".to_string(),
            "13".to_string()]),
            ("61".to_string(), vec!["13".to_string(),
            "53".to_string(),
            "29".to_string()]),
            ("47".to_string(), vec!["53".to_string(),
            "13".to_string(),
            "61".to_string(),
            "29".to_string()]),
            ("75".to_string(), vec!["29".to_string(),
            "53".to_string(),
            "47".to_string(),
            "61".to_string(),
            "13".to_string()]),
            ("97".to_string(), vec!["13".to_string(),
            "61".to_string(),
            "47".to_string(),
            "29".to_string(),
            "53".to_string(),
            "75".to_string()])
        ]);
        let update = vec!["75","47","61","53","29"];

        assert!(is_correctly_ordered(&rules, &update));
    }


    #[test]
    fn test_read_rules() -> Result<()> {
        let data = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n\
        53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13";
        let cursor = Cursor::new(data);
        let mut reader = BufReader::new(cursor);

        let real_map = read_rules(&mut reader)?;
        let test_map = HashMap::from([
            ("29".to_string(), vec!["13".to_string()]),
            ("53".to_string(), vec!["29".to_string(),
            "13".to_string()]),
            ("61".to_string(), vec!["13".to_string(),
            "53".to_string(),
            "29".to_string()]),
            ("47".to_string(), vec!["53".to_string(),
            "13".to_string(),
            "61".to_string(),
            "29".to_string()]),
            ("75".to_string(), vec!["29".to_string(),
            "53".to_string(),
            "47".to_string(),
            "61".to_string(),
            "13".to_string()]),
            ("97".to_string(), vec!["13".to_string(),
            "61".to_string(),
            "47".to_string(),
            "29".to_string(),
            "53".to_string(),
            "75".to_string()])
        ]);

        assert_eq!(test_map, real_map);

        Ok(())
    }
    
}