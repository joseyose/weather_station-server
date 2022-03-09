use actix_web::{web, Result};
use super::NewWeather;

use serde::Deserialize;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

// Pg connections comes from diesel::prelude
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use crate::schema;
use schema::weather;
use schema::weather::dsl::*;

pub async fn new_weather(info: web::Json<NewWeather>, pool: web::Data<DbPool>) -> Result<String> {
    println!("Pushing new weather data to database...");
    let connection = pool.get()
        .expect("Can't get db connection from pool...");

    let new_weather = NewWeather {
        temperature: info.temperature,
        humidity: info.humidity,
        pressure: info.pressure,
        date: info.date,
    };

    web::block(move ||
        diesel::insert_into(weather::table)
            .values(&new_weather)
            .execute(&connection)
    )
        .await
        .unwrap();

    println!("{:?}", info);
    Ok(format!("Pushed to database: {:?}", info))
}