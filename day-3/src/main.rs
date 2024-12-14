use std::path::Path;
use anyhow::Result;

fn main() -> Result<()>{
    let f = day_3::read_file(Path::new("./data/input.txt"))?;
    let v = day_3::filter_corrupted(f)?;
    let result = day_3::multiply(&v)?;
    println!("{}", result);
    Ok(())
}
