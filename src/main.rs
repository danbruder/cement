#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::{Json, Value};

mod pool;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize, Serialize)]
struct NewComment {
    username: String,
    body: String,
}

#[post("/", format = "application/json", data = "<comment>")]
fn new(comment: Json<NewComment>) -> Json<NewComment> {
    comment
}

fn main() {
    rocket::ignite()
        .manage(pool::init_pool())
        .mount("/", routes![index, new])
        .launch();
}
