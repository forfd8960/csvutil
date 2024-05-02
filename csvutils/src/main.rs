use clap::Parser;

use csvutils::cli::options::{
    Options,
    SubCommand::{Deduplicate, Filter, Merge},
};

fn main() {
    let opts = Options::parse();
    println!("{:?}", opts);
    handle_sub_commands(opts);
}

fn handle_sub_commands(opts: Options) {
    match opts.cmd {
        Deduplicate(dedup_opts) => {
            println!("{:?}", dedup_opts);
        }
        Filter(filter_opts) => {
            println!("{:?}", filter_opts);
        }
        Merge(merge_otps) => {
            println!("{:?}", merge_otps);
        }
    }
}
