pub mod pets;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![pets::create_pet_v2, pets::get_pet_v2]
}

