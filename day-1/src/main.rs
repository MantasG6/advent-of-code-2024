use anyhow::Result;

fn main() -> Result<()> {

    let input_path = std::path::Path::new("./data/input.txt");

    let content = day_1::read_file(input_path)?;

    let (l1,l2) = day_1::get_lists(&content)?;

    let dist = day_1::distance(l1, l2)?;

    println!("Lists path: {}\nDistance: {}", input_path.display(), dist);

    Ok(())
}