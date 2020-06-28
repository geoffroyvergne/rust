use std::io::Cursor;
use crate::models::status::{CustomStatus};
use rocket::http::{ContentType, Status};
use rocket::Response;

#[get("/", format = "json")]
pub fn index() -> Response<'static> {
    
    info!("index");

    return Response::build()
        .status(Status::Created)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(serde_json::to_string(&CustomStatus {
            message: String::from("index"),
            code: 201,
        }).unwrap()))
        .finalize();
}

#[get("/test", format = "json")]
pub fn test() -> Response<'static> {
    
    info!("index/test");

    return Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(serde_json::to_string(&CustomStatus {
            message: String::from("index/test"),
            code: 200,
        }).unwrap()))
        .finalize();
}