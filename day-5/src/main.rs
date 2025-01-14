use std::{fs::File, io::BufReader, path::Path};

use anyhow::{Result, Context};

fn main() -> Result<()> {
    let input_path = Path::new("./data/input.txt");
    let file = File::open(input_path)
    .with_context(|| format!("Failed reading file from path {}", input_path.display()))?;
    let mut reader = BufReader::new(file);

    let rules = day_5::read_rules(&mut reader)?;

    let sum = day_5::correctly_ordered_sum(&mut reader, &rules)?;

    println!("SUM:: {}", sum);

    Ok(())
}
