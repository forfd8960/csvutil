use clap::Parser;

#[derive(Debug, Parser)]
pub struct MergeOpts {
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    pub inputs: Vec<String>,
    #[arg(short, long, default_value = "output.csv")]
    pub output: String,
}
