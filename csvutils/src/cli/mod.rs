use std::path::{Path, PathBuf};

pub mod dedup;
pub mod filter;
pub mod merge;
pub mod options;

pub use dedup::DedupOpts;
pub use filter::FilterOpts;
pub use merge::MergeOpts;

fn verify_input(f: &str) -> std::result::Result<String, &'static str> {
    if std::path::Path::new(f).exists() {
        Ok(f.into())
    } else {
        Err("File is not exists")
    }
}

fn verify_output(f: &str) -> std::result::Result<PathBuf, &'static str> {
    let p = Path::new(f);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err("Path is not exists or is not a dir")
    }
}
