use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

use crate::terminator::Terminator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[clap(short, long)]
    pub path: PathBuf,

    #[clap(short = 'l', long, value_enum, default_value_t=Terminator::LF)]
    pub terminator: Terminator,

    #[clap(short, long, default_value_t = false)]
    pub write: bool,

    #[clap(long)]
    pub exclude: Vec<PathBuf>,

    #[clap(short = 'j', long, default_value_t = 1)]
    pub threads: usize,

    #[clap(long, default_value_t = 10)]
    pub max_depth: usize,
}
