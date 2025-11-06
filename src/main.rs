mod args;
mod highlight;
mod search;

use args::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse_args();
    search::run(&args)
}
