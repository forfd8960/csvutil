pub mod dedup;

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
}
