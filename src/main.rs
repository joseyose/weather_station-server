#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

mod views;
mod schema;

// Pg connections comes from diesel::prelude
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port 8000...");

    // Set up database connections
    let database_url = "postgres://postgres:admin@localhost".to_string();

    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool...");

    HttpServer::new(move || {
        let app = App::new()
            .data(pool.clone())
            .configure(views::views_factory);
        return app;
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
