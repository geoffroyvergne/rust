use rocket_contrib::json::{Json};
use crate::models::status::{CustomStatus};

#[get("/echo", format = "json")]
pub fn echo() -> Json<CustomStatus> {
    Json(CustomStatus {
        message: String::from("debug/echo"),
        code: 200,
    })
}

#[get("/ping", format = "json")]
pub fn ping() -> Json<CustomStatus> {
    Json(CustomStatus {
        message: String::from("debug/ping"),
        code: 200,
    })
}