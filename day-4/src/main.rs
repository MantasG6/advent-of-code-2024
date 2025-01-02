use anyhow::{Ok, Result};

fn main() -> Result<()>{
    let input = std::path::Path::new("./data/input.txt");
    
    let xmas_count = day_4::xmas_count(input)?;

    println!("XMAS_COUNT: {}", xmas_count);
    
    Ok(())
}
