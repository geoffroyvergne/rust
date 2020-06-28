use rocket_contrib::json::{Json};
use crate::models::status::{CustomStatus};

#[catch(404)]
pub fn not_found() -> Json<CustomStatus> {
    Json(CustomStatus {
        message: String::from("Not found"),
        code: 404,
    })
}
