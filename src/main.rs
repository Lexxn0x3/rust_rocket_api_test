#[macro_use] extern crate rocket;
use rocket_cors::{CorsOptions};

mod api;

#[launch]
fn rocket() -> _ {
    // Customize CORS options if necessary
    let cors_options = CorsOptions::default()
        .to_cors()
        .expect("CorsOptions::to_cors() failed");

    rocket::build()
        .mount("/api/v1", api::v1::routes())
        .mount("/api/v2", api::v2::routes())
        .attach(cors_options)
}
