use actix_web::web;

mod new_entry;
mod read;
mod hello;
use super::path::Path;
use super::structs::new_weather::NewWeather;
use super::structs::weather::Weather;

pub fn weather_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path{ prefix: String::from("/weather") };

    app.route(&base_path.define(String::from("/new-entry")),
              web::post().to(new_entry::new_weather));

    app.route(&base_path.define(String::from("/read")),
        web::get().to(read::read_db));

    app.route(&base_path.define(String::from("/hello")),
        web::get().to(hello::hello));

        
}