use anyhow::Result;

/// Find the difference between 2 lists in the file.
fn main() -> Result<()> {

    let input_path = std::path::Path::new("./data/input_test_11.txt");

    let content = day_1::read_file(input_path)?;

    let (l1,l2) = day_1::get_lists(&content)?;

    println!("List1: ");
    for num in l1 {
        print!("{} ", num)
    }
    println!();

    println!("List2: ");
    for num in l2 {
        print!("{} ", num)
    }
    println!();

    Ok(())
}