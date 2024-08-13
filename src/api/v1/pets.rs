use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct Pet {
    pub id: u32,
    pub name: String,
    pub species: String,
}

#[post("/pet", format = "json", data = "<pet>")]
pub fn create_pet(pet: Json<Pet>) -> Json<Pet> {
    Json(pet.into_inner())
}

#[get("/pet/<id>")]
pub fn get_pet(id: u32) -> Json<Pet> {
    Json(Pet { id, name: "Buddy".to_string(), species: "Dog".to_string() })
}
