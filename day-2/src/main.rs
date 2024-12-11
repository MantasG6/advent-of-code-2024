use anyhow::Result;

fn main() -> Result<()> {
    let input_path = std::path::Path::new("./data/input_test_2.txt");

    let content = day_2::read_file(input_path)?;
    println!("{}", content);

    Ok(())
}
