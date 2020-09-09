use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::SaasDbConn;
use super::team::Team;

#[post("/", data = "<team>")]
fn create(team: Json<Team>, connection: SaasDbConn) -> Json<Team> {
    let insert = Team { id: None, ..team.into_inner() };
    Json(Team::create(insert, &connection))
}

#[get("/")]
fn read(connection: SaasDbConn) -> Json<JsonValue> {
    Json(json!(Team::read(&connection)))
}

#[put("/<id>", data = "<team>")]
fn update(id: i32, team: Json<Team>, connection: SaasDbConn) -> Json<JsonValue> {
    let update = Team { id: Some(id), ..team.into_inner() };
    Json(json!({
        "success": Team::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: SaasDbConn) -> Json<JsonValue> {
    Json(json!({
        "success": Team::delete(id, &connection)
    }))
}

pub fn routes() -> Vec<rocket::Route> {
  routes![read, create, update, delete]
}