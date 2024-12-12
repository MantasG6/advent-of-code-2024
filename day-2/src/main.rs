use std::io::Read;
use anyhow::Result;

fn main() -> Result<()> {
    let input_path = std::path::Path::new("./data/input_test_2.txt");

    let mut file = day_2::read_file(input_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let safe = day_2::safe_report("1 2 3 4")?;
    println!("{}", safe);

    Ok(())
}
