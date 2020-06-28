#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate log;

pub mod lib;
//pub mod cors;
pub mod models;
pub mod routes;

fn main() {
    rocket::ignite()
        .mount("/index", routes![routes::index::index, routes::index::test, routes::index::notfound, routes::index::code])
        .mount("/debug", routes![routes::debug::echo, routes::debug::ping])
        .register(catchers![routes::errors::not_found])
        .attach(lib::cors::CorsFairing)
        .launch();
}
