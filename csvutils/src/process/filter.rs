use std::collections::HashMap;

use anyhow::anyhow;

use super::{read_csv, write_csv, CSVResult};

pub fn process_filter(input_csv: String, output: String, exps: Vec<String>) -> anyhow::Result<()> {
    if exps.len() <= 0 {
        return Err(anyhow!("empty exps"));
    }

    // parse exps and build column to exp value map
    let cols = parse_exps(exps)?;
    println!("fields: {:?}", cols); // {"c1": v1, "c2": "v2"}

    // idx_value contains the index to the  in the csv
    let mut idx_value = HashMap::new();

    // build idx to exp_value mapping from csv headers
    let csv_data = read_csv(&input_csv)?;
    for (idx, hdr) in csv_data.headers.iter().enumerate() {
        let key = hdr.as_str();
        if !cols.contains_key(key) {
            continue;
        }

        idx_value.insert(idx, cols.get(key).unwrap());
    }
    println!("idx_value: {:?}", idx_value); // {1: "val1", 0: "val2"}

    let mut data: Vec<Vec<String>> = Vec::with_capacity(csv_data.data.len());

    // iter over the csv data and for each record in the data
    // * if one of the idx(col) in the record(r) contains the val, then put the record in the final data
    // * if not skip the record(r)
    for r in csv_data.data {
        let mut contains_data = false;

        for (idx, d) in r.iter().enumerate() {
            if let Some(exp_val) = idx_value.get(&idx) {
                if d.contains(*exp_val) {
                    contains_data = true;
                }
            };
        }
        if contains_data {
            data.push(r);
        }
    }

    write_csv(output, CSVResult::new(data, csv_data.headers))?;

    anyhow::Ok(())
}

fn parse_exps(exps: Vec<String>) -> anyhow::Result<HashMap<String, String>> {
    let mut cols = HashMap::new();
    for e in exps {
        let field_value: Vec<String> = e.split('=').map(|v| v.to_string()).collect();

        if field_value.len() < 2 {
            return Err(anyhow!("bad exp: {}", e));
        }

        cols.insert(field_value[0].clone(), field_value[1].clone());
    }

    anyhow::Ok(cols)
}
