use clap::Parser;
use anyhow::{Context, Ok, Result};
use indicatif::ProgressBar;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<()>{
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
                .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let lines: Vec<_> = content.lines().collect();
    let pb = ProgressBar::new(lines.len() as u64);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    pb.finish_with_message("Busqueda completa");

    Ok(())
}