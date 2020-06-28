#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;

pub mod lib;
use crate::lib::mongo::{MongoDbConn};

#[macro_use(bson, doc)] extern crate bson;

// https://github.com/SergioBenitez/Rocket/blob/master/examples/json/src/main.rs
// https://api.rocket.rs/v0.4/rocket_contrib/databases/index.html
// https://docs.rs/mongodb/0.3.12/mongodb/
// https://github.com/zonyitoo/bson-rs

pub mod cors;
pub mod routes;
pub mod models;

fn main() {
    rocket::ignite()
        .mount("/index", routes![routes::index::index, routes::index::test])
        .mount("/debug", routes![routes::debug::echo, routes::debug::ping])
        .mount("/users", routes![routes::users::get, routes::users::add])
        .register(catchers![routes::errors::not_found])
        .attach(cors::CorsFairing)
        .attach(MongoDbConn::fairing())
        .launch();
}
