use clap::Parser;

use super::verify_input;

#[derive(Debug, Parser)]
pub struct FilterOpts {
    #[arg(short, long, value_parser = verify_input)]
    pub input: String,
    #[arg(short, long, default_value = "output.csv")]
    pub output: String,

    /// if field1 contains value1 and field2 contaiins value2
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    pub exps: Vec<String>,
}
