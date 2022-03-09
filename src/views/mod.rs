use actix_web::web;

pub mod structs;
pub use structs::new_weather::NewWeather;

mod weather;
mod path;

pub fn views_factory(app: &mut web::ServiceConfig) {
    weather::weather_factory(app);
}