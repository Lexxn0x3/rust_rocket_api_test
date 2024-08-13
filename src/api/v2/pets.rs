use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct Pet {
    pub id: u32,
    pub name: String,
    pub species: String,
}

#[post("/pet", format = "json", data = "<pet>")]
pub fn create_pet_v2(pet: Json<Pet>) -> Json<Pet> {
    // New behavior or data structure for v2
    Json(pet.into_inner())
}

#[get("/pet/<id>")]
pub fn get_pet_v2(id: u32) -> Json<Pet> {
    // New behavior or data structure for v2
    Json(Pet { id, name: "Max".to_string(), species: "Cat".to_string() })}
