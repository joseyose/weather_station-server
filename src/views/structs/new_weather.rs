use chrono::{DateTime, Local};
use serde::Deserialize;

use crate::schema;
use schema::weather;
use schema::weather::dsl::*;

#[derive(Debug, Deserialize, Insertable)]
#[table_name="weather"]
pub struct NewWeather {
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub date: DateTime<Local>,
}