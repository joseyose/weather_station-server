use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize, Queryable)]
pub struct Weather {
    id: i32,
    temperature: f64,
    humidity: f64,
    pressure: f64,
    date: NaiveDateTime,
}