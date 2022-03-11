use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize, Queryable)]
pub struct Weather {
    id: i32,
    temperature_c: f64,
    temperature_f: f64,
    humidity: f64,
    pressure: f64,
    date: NaiveDateTime,
}