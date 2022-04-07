use clap::Parser;
use colored::*;
use std::fs::{self, metadata, File};
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

const COPY_DIR: &'static str = ".copy";
const SPDX_LICENSE_PREFIX: &'static str = r#"// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts
"#;
const SOLIDITY_EXT: &'static str = "sol";

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, required = true, help = "directory path")]
    dir: String,
    #[clap(
        short,
        long,
        help = "Write this option if you want to add SPDX license at the top of the source file"
    )]
    license: bool,
    #[clap(
        long,
        help = "Write this option if you want to revert the editted files"
    )]
    revert: bool,
    #[clap(long, help = "Write this option if you want to overwrite it")]
    overwrite: bool,
    #[clap(long, required = true, help = "Write a version to be editted")]
    version: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn edit_pragma(file_path: &str, version: &str, overwrite: bool) {
    let mut new_lines: String = read_lines(file_path)
        .unwrap()
        .map(|line| {
            line.map(|stmt| {
                if stmt.contains("pragma") {
                    return format!("pragma solidity {};", version);
                } else {
                    stmt
                }
            })
            .unwrap()
        })
        .collect::<Vec<String>>()
        .join("\n");

    new_lines = format!("{}\n{}", SPDX_LICENSE_PREFIX, new_lines);

    let file_path = if overwrite {
        file_path.to_string()
    } else {
        format!("{}.new", file_path)
    };
    let mut file = File::create(&file_path).unwrap();
    file.write_all(new_lines.as_bytes()).unwrap();
    println!("written to {}", &file_path.yellow());
}

fn search_dirs(dir: &str, version: &str, overwrite: bool) {
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();
        if metadata(path_str).unwrap().is_dir() {
            search_dirs(path_str, version, overwrite);
        } else {
            if let Some(ext) = path.extension() {
                if ext == SOLIDITY_EXT {
                    edit_pragma(path_str, version, overwrite);
                }
            }
        }
    }
}

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
fn main() {
    let args = Args::parse();
    if args.revert {
        copy_dir(COPY_DIR, &args.dir);
        println!("{}", "reverted".red());
        return;
    }
    if !Path::new(COPY_DIR).exists() {
        copy_dir(&args.dir, COPY_DIR);
    }
    search_dirs(&args.dir, &args.version, args.overwrite);
}
