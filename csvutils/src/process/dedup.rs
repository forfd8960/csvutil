use std::collections::HashSet;

use anyhow::anyhow;

use crate::process::read_csv;

pub fn process_dedup(input: &str, field: &str) -> anyhow::Result<Vec<Vec<String>>> {
    let mut csv_result = read_csv(input)?;

    let pos = csv_result.headers.iter().position(|x| x == field);
    match pos {
        Some(idx) => {
            let mut data_set = HashSet::new();
            csv_result.data.retain(|r| data_set.insert(r[idx].clone()));
            anyhow::Ok(csv_result.data)
        }
        None => return Err(anyhow!("field not found")),
    }
}
