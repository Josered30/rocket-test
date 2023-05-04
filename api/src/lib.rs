use api::routes;
use cores::database;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;

mod api;
mod cores;
mod domain;
mod infrastructure;

#[rocket::main]
async fn app() -> Result<(), rocket::Error> {
    dotenv().ok();

    let app = rocket::build()
        .attach(database::database_stage())
        .attach(routes::user_stage())
        .launch()
        .await?;

    Ok(())
}

pub fn main() -> Result<(), rocket::Error> {
    app()
}
