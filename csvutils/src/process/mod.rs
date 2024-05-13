pub mod dedup;
pub mod filter;
pub mod merge;

#[derive(Debug, PartialEq)]
pub struct CSVResult {
    pub data: Vec<Vec<String>>,
    pub headers: Vec<String>,
}

impl CSVResult {
    pub fn new(data: Vec<Vec<String>>, headers: Vec<String>) -> Self {
        Self { data, headers }
    }
}

pub fn read_csv(input: &str) -> anyhow::Result<CSVResult> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut data = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    println!("{:?}", headers);

    for record in reader.records() {
        let r = record?;
        let value = r.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        data.push(value);
    }

    let hdrs = headers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    anyhow::Ok(CSVResult::new(data, hdrs))
}

pub fn write_csv(path: String, csv_data: CSVResult) -> anyhow::Result<()> {
    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(csv_data.headers)?;

    for data in csv_data.data {
        writer.write_record(data)?;
    }
    writer.flush()?;

    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv() -> anyhow::Result<()> {
        let input = "test.csv";
        let result = read_csv(input)?;
        assert_eq!(
            result,
            CSVResult::new(
                vec![
                    vec!["a".to_string(), "b".to_string(), "c".to_string()],
                    vec!["d".to_string(), "e".to_string(), "f".to_string()]
                ],
                vec!["f1".to_string(), "f2".to_string(), "f3".to_string()]
            )
        );
        anyhow::Ok(())
    }

    #[test]
    fn test_write_csv() -> anyhow::Result<()> {
        let input = "test1.csv";
        let result = write_csv(
            input.to_string(),
            CSVResult::new(
                vec![
                    vec!["111".to_string(), "22".to_string(), "333".to_string()],
                    vec!["888".to_string(), "999".to_string(), "666".to_string()],
                ],
                vec!["f1".to_string(), "f2".to_string(), "f3".to_string()],
            ),
        );
        assert!(result.is_ok());
        anyhow::Ok(())
    }
}
