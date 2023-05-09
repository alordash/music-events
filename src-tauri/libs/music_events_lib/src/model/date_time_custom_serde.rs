use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer, Serializer};

pub const DATE_TIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M";

pub fn serialize_naive_date_time<S>(
    naive_date_time: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&naive_date_time.format(DATE_TIME_FORMAT).to_string())
}

pub fn deserialize_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let date_time_string: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&date_time_string, DATE_TIME_FORMAT).map_err(de::Error::custom)
}
