use std::io::Cursor;
use crate::models::status::{CustomStatus};
use rocket::http::{ContentType, Status};
use rocket::Response;

// https://abishov.com/2017/08/08/hexocat-bot-part-2.html

#[get("/", format = "json")]
pub fn index() -> Response<'static> {
    
    info!("index");

    let custom_status = serde_json::to_string(&CustomStatus {
        message: String::from("index"),
        code: 201,
    }).unwrap();

    return Response::build()
        .status(Status::Created)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(custom_status))
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

#[get("/notfound", format = "json")]
pub fn notfound() -> Response<'static> {

    info!("index/notfound");

    return Response::build()
        .status(Status::NotFound)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(serde_json::to_string(&CustomStatus {
            message: String::from("index/notfound"),
            code: 404,
        }).unwrap()))
        .finalize();
}

#[get("/code/<code>", format = "json")]
pub fn code(code: u16) -> Response<'static> {
    
    info!("index/code/{}", code);
    
    let status: Status = match code {
        100 ... 103 => Status::Continue,
        200 ... 226 => Status::Ok,
        300 ... 310 => Status::Found,
        400 ... 499 => Status::BadRequest, 
        500 ... 527 => Status::InternalServerError,
        _ => Status::NotFound,
    };

    return Response::build()
        .status(status)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(serde_json::to_string(&CustomStatus {
            message: String::from(status.reason),
            code: status.code,
        }).unwrap()))
        .finalize();
}
