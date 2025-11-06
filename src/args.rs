use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(name = "sniff")]
pub struct Args {
    /// The regex pattern to search for
    pub pattern: String,

    /// Path to search (file or directory)
    #[arg(default_value = ".")]
    pub path: String,

    /// Ignore case distinctions
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Disable recursive search
    #[arg(short = 'R', long = "no-recursive", action = ArgAction::SetTrue)]
    pub no_recursive: bool,

    /// Limit number of matches
    #[arg(short, long)]
    pub max: Option<usize>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
