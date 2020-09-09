#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

mod users;
mod teams;
mod db;

#[database("saas")]
struct SaasDbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
    .attach(SaasDbConn::fairing())
    .mount("/", routes![index])
    .mount("/users", users::routes::routes())
    .mount("/teams", teams::routes::routes())
    .launch();
}