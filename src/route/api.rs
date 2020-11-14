use rocket_contrib::json::Json;
use crate::item_generator::resources::Object;

#[get("/api/testobjectgenerationlol")]
pub fn testobjectgenerationlol() -> Json<Object> {
    use crate::item_generator::generator::generate_loot;
    let loot = generate_loot();
    Json(loot)
}