use clap::Parser;

use super::{DedupOpts, FilterOpts, MergeOpts};

#[derive(Debug, Parser)]
#[command(name="csvutils", version, author, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "merge", about = "merge multi csv file into one")]
    Merge(MergeOpts),
    #[command(name = "filter", about = "filter records by condition")]
    Filter(FilterOpts),
    #[command(name = "dedup", about = "deduplicate rows by field")]
    Deduplicate(DedupOpts),
}
