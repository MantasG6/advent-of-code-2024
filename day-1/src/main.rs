use anyhow::Result;

/// Find the difference between 2 lists in the file.
fn main() -> Result<()> {

    let input_path = std::path::Path::new("./data/input_test_11.txt");

    let lists = day_1::read_file(input_path)?;

    println!("{}", lists);

    Ok(())
}