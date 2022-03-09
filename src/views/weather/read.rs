use actix_web::{web, Result};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
// Pg connections comes from diesel::prelude
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use crate::schema;
use schema::weather;
use schema::weather::dsl::*;
use crate::views::structs::weather::Weather;

pub async fn read_db(pool: web::Data<DbPool>) -> Result<String> {
    let connection = pool.get()
        .expect("Can't get db connection from pool...");

    let weather_data = web::block(move || {
        weather.limit(100).load::<Weather>(&connection)
    })
        .await
        .unwrap();

    for i in &weather_data {
        println!("{:?}", i);
    };

    Ok(format!("Read was successful..."))
}