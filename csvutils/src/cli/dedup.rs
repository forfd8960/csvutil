use clap::Parser;

use super::verify_input;

#[derive(Debug, Parser)]
pub struct DedupOpts {
    #[arg(short, long, value_parser = verify_input)]
    pub input: String,
    /// which field need to be deduplicated
    #[arg(short, long)]
    pub field: String,
    #[arg(short, long, default_value = "output.csv")]
    pub output: Option<String>,
}
