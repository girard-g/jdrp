use rocket_contrib::json::Json;
use crate::item_generator::resources::{Loot};
use crate::item_generator::monster::{Monster};

#[get("/api/testobjectgenerationlol")]
pub fn testobjectgenerationlol() -> Json<Loot> {
    use crate::item_generator::generator::generate_loot;
    let loot = generate_loot();
    Json(loot)
}

#[get("/api/testmonstergeneration/<monster>")]
pub fn testmonstergeneration(monster: String) -> Json<Monster> {
    use crate::item_generator::generator::generate_monster;
    let loot = generate_monster(monster);
    Json(loot)
}