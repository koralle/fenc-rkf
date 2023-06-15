use crate::eol::replace_terminator;
use crate::fenc::to_utf8;
use crate::terminator::{self, Terminator};
use anyhow::Result;
use std::convert::AsRef;
use std::fs::{self, File};
use std::io::Read;
use std::io::{stdout, Write};
use std::path::Path;
use std::process::ExitCode;
use std::result;
use std::sync::atomic::{AtomicBool, Ordering};

use ignore::WalkBuilder;

pub fn scan<P: AsRef<Path>>(
    path: P,
    terminator: &Terminator,
    write: bool,
    threads: usize,
    max_depth: usize,
) -> Result<()> {
    let mut walker = WalkBuilder::new(path);

    walker
        .hidden(true)
        .git_ignore(true)
        .same_file_system(true)
        .follow_links(true)
        .max_depth(Some(max_depth));

    for result in walker.build() {
        match result {
            Ok(dir_entry) => {
                if let Some(filetype) = dir_entry.file_type() {
                    if !filetype.is_file() {
                        continue;
                    }
                }

                println!(
                    "{}: {}",
                    dir_entry.path().to_str().unwrap(),
                    dir_entry.depth()
                );

                let absolute_path = fs::canonicalize(dir_entry.path()).unwrap();

                let s = fs::read(&absolute_path).unwrap();

                let utf8_text = to_utf8(&s).unwrap();
                let eolified_text = replace_terminator(&utf8_text, terminator);

                if write {
                    fs::write(absolute_path, eolified_text)?;
                } else {
                }
            }
            _ => (),
        }
    }

    Ok(())
}
