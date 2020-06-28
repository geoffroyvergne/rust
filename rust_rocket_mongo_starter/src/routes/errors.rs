use rocket_contrib::json::{Json};
use crate::models::status::{Status};

#[catch(404)]
pub fn not_found() -> Json<Status> {
    Json(Status {
        message: String::from("Not found"),
        code: 404,
    })
}
