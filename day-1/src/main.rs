use anyhow::Result;

fn main() -> Result<()> {

    let input_path = std::path::Path::new("./data/input.txt");

    let content = day_1::read_file(input_path)?;

    let (l1,l2) = day_1::get_lists(&content)?;

    let diff = day_1::difference(l1, l2)?;

    println!("Lists path: {}\nDifference: {}", input_path.display(), diff);

    Ok(())
}