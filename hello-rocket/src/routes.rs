
use rocket_contrib::json::{Json};
use crate::models::{Status};

#[get("/", format = "json")]
pub fn index() -> Json<Status> {
    Json(Status {
        message: String::from("index"),
        code: 200,
    })
}

#[get("/world")]              // <- route attribute
pub fn world() -> &'static str {  // <- request handler
    "Hello, world!"
}

#[catch(404)]
pub fn not_found() -> Json<Status> {
    Json(Status {
        message: String::from("Not found"),
        code: 404,
    })
}