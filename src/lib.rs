use anyhow::{Context, Result};

pub fn find_matches(bufreader: impl std::io::BufRead, pattern: &str, mut out: impl std::io::Write) -> Result<()> {

    if pattern.is_empty() {
        return Err(anyhow::anyhow!("Empty pattern"))
    }

    for line in bufreader.lines() {
        if let Ok(line) = line {
            if line.contains(pattern) {
                writeln!(out, "{}", line)
                    .with_context(|| format!("Failed to write line {}", line))?;
            }
        }
    }
    Ok(())
}

#[test]
fn test_find_matches() {
    let mut result = Vec::new();
    let bufreader = std::io::BufReader::new("lorem ipsum\ndolor sit amet".as_bytes());
    assert!(find_matches(bufreader, "lorem", &mut result).is_ok());
    assert_eq!(result, b"lorem ipsum\n");
}

