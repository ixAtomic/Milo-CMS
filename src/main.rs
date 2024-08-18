use application::traits::queryable::Queryable;
use domain::enums::drivers::DriverKind;
use dotenv::dotenv;
use infrastructure::enums::data_access::DataAccess;
use infrastructure::mysql::access::MySQLAccess;
use infrastructure::postgres::access::PostgresAccess;
use presentation::collection_endpoints::get_collection_by_id;
use presentation::collection_endpoints::get_collections;
use presentation::field_endpoints::get_fields_by_collection;
use std::env;
use std::str::FromStr;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("CONNECTION_STRING").expect("Connection String must be supplied");

    let db = match DriverKind::from_str(
        &env::var("DATABASE_TYPE")
            .expect("DATABASE_TYPE should be provided as an environment variable"),
    ) {
        Ok(db) => db,
        Err(_) => {
            eprintln!("DATABASE_TYPE is not an option");
            std::process::exit(1);
        }
    };
    let builder;
    match db {
        DriverKind::Postgres => {
            let pg = match PostgresAccess::new(&database_url).await {
                Ok(pg) => pg,
                Err(_) => {
                    eprintln!("Failed to Start Postgres Database");
                    std::process::exit(1);
                }
            };

            builder = rocket::build().manage(DataAccess::Postgres(pg));
        }
        DriverKind::MSSQL => todo!(),
        DriverKind::MySql => {
            let my = match MySQLAccess::new(&database_url).await {
                Ok(my) => my,
                Err(_) => {
                    eprintln!("Failed to Start MySQL Database");
                    std::process::exit(1);
                }
            };

            builder = rocket::build().manage(DataAccess::MYSQL(my));
        }
        DriverKind::MongoDB => todo!(),
        DriverKind::Sqlite => todo!(),
    }

    builder
        .mount(
            "/collections",
            routes![get_collections, get_collection_by_id],
        )
        .mount("/fields", routes![get_fields_by_collection])
}
