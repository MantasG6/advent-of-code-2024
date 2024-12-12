use anyhow::Result;

fn main() -> Result<()> {
    let input_path = std::path::Path::new("./data/input.txt");

    let file = day_2::read_file(input_path)?;

    let num_safe_reports = day_2::safe_reports_number(file)?;
    println!("{}", num_safe_reports);

    Ok(())
}
