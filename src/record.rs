use chrono::naive::NaiveDate;
use chrono::format::ParseError;
use serde::{Deserialize, Deserializer};

fn parse_parse(str: &String) -> Result<NaiveDate, ParseError> {
    return NaiveDate::parse_from_str(str, "%m/%d/%Y");
}

fn callback<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>

where
    D: Deserializer<'de>,
{
    let y = parse_parse(&String::deserialize(deserializer)?);
    y.map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
pub struct Record {
    #[serde(deserialize_with = "callback")]
    date: NaiveDate,
    transaction: String,
    name: String,
    memo: Option<String>,
    amount: f64
}
