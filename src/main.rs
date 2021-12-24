use std::io::BufReader;

use anyhow::{Context, Result};

use structopt::StructOpt;

/// Search for a pattern in the file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let file = std::fs::File::open(&args.path)
        .with_context(|| format!("could not open file {:#?}", &args.path))?;

    let bufreader = BufReader::new(file);
    grrs_64bit::find_matches(bufreader, &args.pattern, std::io::stdout())?;
    Ok(())
}
