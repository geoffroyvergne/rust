use std::io::Cursor;
use rocket_contrib::json::{Json};
use crate::models::status::{CustomStatus};
use crate::models::users::{Users};

use rocket::http::{ContentType, Status};
use rocket::Response;

use rocket_contrib::databases::mongodb;
use mongodb::{Bson, doc};
use crate::rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::lib::mongo::{MongoDbConn};

use bson;

#[get("/", format = "json")]
pub fn get(conn: MongoDbConn) -> Response<'static> {

    info!("users get all");

    let mut users_list :Vec<Users> = Vec::new();

    let cursor = conn.collection("users").find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            let user : Users = bson::from_bson(bson::Bson::Document(item))
            .expect("failed to deserialize");
            users_list.push(user);
        }
    }
    
    return Response::build()
        .status(Status::Created)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(serde_json::to_string(&users_list).unwrap()))
        .finalize();
}

#[post("/", format = "json", data = "<user>")]
pub fn add(conn: MongoDbConn, user: Json<Users>) -> Response<'static> {

    info!("users post");
    
    if let bson::Bson::Document(document) = bson::to_bson(&user.0)
        .expect("failed to serialize") {
            conn.collection("users")
                .insert_one(document, None)
                .expect("failed to insert");  
    } else {
        error!("Error converting the BSON object into a MongoDB document");
    }

    return Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(serde_json::to_string(&CustomStatus {
            message: String::from("user/add"),
            code: 200,
        }).unwrap()))
        .finalize();
}