use clap::Parser;

use csvutils::{
    cli::options::{
        Options,
        SubCommand::{Deduplicate, Filter, Merge},
    },
    process,
};

fn main() -> anyhow::Result<()> {
    let opts = Options::parse();
    println!("{:?}", opts);
    handle_sub_commands(opts)
}

fn handle_sub_commands(opts: Options) -> anyhow::Result<()> {
    match opts.cmd {
        Deduplicate(dedup_opts) => {
            println!("{:?}", dedup_opts);
            let result = process::dedup::process_dedup(&dedup_opts.input, &dedup_opts.field)?;
            println!("process_dedup: {:?}", result);
        }
        Filter(filter_opts) => {
            println!("{:?}", filter_opts);
        }
        Merge(merge_otps) => {
            println!("{:?}", merge_otps);
        }
    }

    anyhow::Ok(())
}
