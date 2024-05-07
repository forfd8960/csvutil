use std::collections::HashSet;

use anyhow::anyhow;

use super::read_csv;

pub fn process_dedup(input: &str, field: &str) -> anyhow::Result<Vec<Vec<String>>> {
    let csv_result = read_csv(input)?;

    let mut field_idx: usize = 0;
    let pos = csv_result.headers.iter().position(|x| x == field);
    match pos {
        Some(idx) => field_idx = idx,
        None => return Err(anyhow!("field not found")),
    }

    let mut dedup_data: Vec<Vec<String>> = Vec::with_capacity(csv_result.data.len() as usize);
    let mut data_set = HashSet::new();
    for record in csv_result.data {
        let r = record.clone();
        let v = r[field_idx].clone();
        if data_set.contains(&v) {
            continue;
        }

        data_set.insert(v);
        dedup_data.push(record);
    }

    anyhow::Ok(dedup_data)
}
