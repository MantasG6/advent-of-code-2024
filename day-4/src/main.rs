use anyhow::{Ok, Result};

fn main() -> Result<()>{
    let input = std::path::Path::new("./data/input.txt");
    
    let xmas_count = day_4::xmas_count(input)?;

    println!("X-MAS_COUNT: {}", xmas_count);

    Ok(())
}
