pub mod pets;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![pets::create_pet, pets::get_pet]
}

