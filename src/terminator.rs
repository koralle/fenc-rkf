use anyhow::{anyhow, Error, Result};
use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone, PartialEq, Eq)]
pub enum Terminator {
    LF,
    CRLF,
}
