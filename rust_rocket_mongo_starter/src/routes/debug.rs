use std::io::Cursor;
use crate::models::status::{CustomStatus};
use rocket::http::{ContentType, Status};
use rocket::Response;

#[get("/echo", format = "json")]
pub fn echo() -> Response<'static> {
    
    info!("debug/echo");

    return Response::build()
        .status(Status::Created)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(serde_json::to_string(&CustomStatus {
            message: String::from("debug/echo"),
            code: 200,
        }).unwrap()))
        .finalize();
}

#[get("/ping", format = "json")]
pub fn ping() -> Response<'static> {
    
    info!("debug/ping");

    return Response::build()
        .status(Status::Created)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(serde_json::to_string(&CustomStatus {
            message: String::from("debug/ping"),
            code: 200,
        }).unwrap()))
        .finalize();
}