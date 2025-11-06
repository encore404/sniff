use crate::{args::Args, highlight};
use colored::Colorize;
use regex::RegexBuilder;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(args: &Args) -> anyhow::Result<()> {
    // Build regex with optional case-insensitive mode
    let re: regex::Regex = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()?;

    let walker: walkdir::WalkDir = if args.no_recursive {
        walkdir::WalkDir::new(&args.path).max_depth(1)
    } else {
        walkdir::WalkDir::new(&args.path)
    };

    let mut match_count: usize = 0;

    for entry in walker
        .into_iter()
        .filter_map(|e: Result<walkdir::DirEntry, walkdir::Error>| e.ok())
        .filter(|e: &walkdir::DirEntry| e.file_type().is_file())
    {
        let file: File = File::open(entry.path())?;
        let mut reader: BufReader<File> = BufReader::new(file);
        let mut buf: Vec<u8> = Vec::new();
        let mut line_number: usize = 0usize;

        loop {
            buf.clear();

            let bytes_read: usize = reader.read_until(b'\n', &mut buf)?;
            if bytes_read == 0 {
                break; // End of file
            }

            line_number += 1;

            let line: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf);
            let line = line.trim_end();

            if re.is_match(&line) {
                let highlighted: String = highlight::highlight_line(&line, &re);

                println!(
                    "{}({}): {}",
                    entry.path().display().to_string().bright_blue(),
                    line_number.to_string().yellow(),
                    highlighted
                );

                match_count += 1;

                if let Some(max) = args.max {
                    if match_count >= max {
                        return Ok(());
                    }
                }
            }
        }
    }

    Ok(())
}
