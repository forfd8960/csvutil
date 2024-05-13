use super::{read_csv, write_csv, CSVResult};

pub fn process_merge(inputs: Vec<String>, output: String) -> anyhow::Result<()> {
    let mut results = Vec::new();
    let mut headers = Vec::new();
    for input in inputs {
        let csv_result = read_csv(&input)?;
        headers = csv_result.headers.clone();
        results.push(csv_result.data);
    }

    let data: Vec<_> = results.into_iter().map(|v| v).flatten().collect();

    write_csv(output, CSVResult::new(data, headers))?;
    anyhow::Ok(())
}
