mod cli;
mod eol;
mod fenc;
mod terminator;
mod walk;

use anyhow::{anyhow, Result};
use clap::Parser;

fn run(cli: &cli::Cli) -> anyhow::Result<()> {
    let result = walk::scan(
        &cli.path,
        &cli.terminator,
        cli.write,
        cli.threads,
        cli.max_depth,
    );

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    match run(&args) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("[#rkf] something error!: {}", e)),
    }
}
