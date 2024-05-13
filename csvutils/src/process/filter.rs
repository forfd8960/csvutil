use std::collections::HashMap;

use anyhow::anyhow;

use super::{read_csv, write_csv, CSVResult};

pub fn process_filter(input_csv: String, output: String, exps: Vec<String>) -> anyhow::Result<()> {
    if exps.len() <= 0 {
        return Err(anyhow!("empty exps"));
    }

    let fieds = parse_exps(exps)?;

    let mut idx_value = HashMap::new();

    let csv_data = read_csv(&input_csv)?;
    for (idx, hdr) in csv_data.headers.iter().enumerate() {
        let key = hdr.as_str();
        if !fieds.contains_key(key) {
            continue;
        }

        idx_value.insert(idx, fieds.get(key).unwrap());
    }

    let mut data: Vec<Vec<String>> = Vec::with_capacity(csv_data.data.len());
    for r in csv_data.data {
        let mut contains_data = false;

        for (idx, d) in r.iter().enumerate() {
            let v = idx_value.get(&idx).unwrap();
            if d.contains(*v) {
                contains_data = true;
            }
        }
        if contains_data {
            data.push(r);
        }
    }

    write_csv(output, CSVResult::new(data, csv_data.headers))?;

    anyhow::Ok(())
}

fn parse_exps(exps: Vec<String>) -> anyhow::Result<HashMap<String, String>> {
    let mut fieds = HashMap::new();
    for e in exps {
        let field_value: Vec<String> = e.split('=').map(|v| v.to_string()).collect();

        if field_value.len() < 2 {
            return Err(anyhow!("bad exp: {}", e));
        }

        fieds.insert(field_value[0].clone(), field_value[1].clone());
    }

    anyhow::Ok(fieds)
}
