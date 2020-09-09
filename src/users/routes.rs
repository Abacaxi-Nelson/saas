use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use crate::SaasDbConn;
use super::user::User;



#[post("/", data = "<user>")]
fn create(user: Json<User>, connection: SaasDbConn) -> Json<User> {
    let insert = User { id: None, ..user.into_inner() };
    Json(User::create(insert, &connection))
}

#[get("/")]
fn read(connection: SaasDbConn) -> Json<JsonValue> {
    Json(json!(User::read(&connection)))
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>, connection: SaasDbConn) -> Json<JsonValue> {
    let update = User { id: Some(id), ..user.into_inner() };
    Json(json!({
        "success": User::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: SaasDbConn) -> Json<JsonValue> {
    Json(json!({
        "success": User::delete(id, &connection)
    }))
}

pub fn routes() -> Vec<rocket::Route> {
  routes![read, create, update, delete]
}