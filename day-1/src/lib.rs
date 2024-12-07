use anyhow::{Context, Error, Result};

pub fn read_file(path: &std::path::Path) -> Result<String, Error> {
    let contents = std::fs::read_to_string(path)
    .with_context(|| format!("could not read file {}", path.display()))?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::*;
    use anyhow::Result;
    use crate::read_file;


    #[test]
    fn test_read_file_success() -> Result<()> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("A test\nActual content\nMore content\nAnother test")?;

        let result = read_file(file.path())?;
        assert_eq!(result, "A test\nActual content\nMore content\nAnother test");
        Ok(())
    }

    #[test]
    fn test_read_file_negative() -> Result<()> {
        let result = read_file(std::path::Path::new("sample.txt"));
        assert_eq!(result.is_err_and(|e| e.to_string().contains("could not read file")), true);
        Ok(())
    }
}