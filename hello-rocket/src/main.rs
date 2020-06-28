#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

// https://github.com/SergioBenitez/Rocket/blob/master/examples/json/src/main.rs
// https://api.rocket.rs/v0.4/rocket_contrib/databases/index.html

pub mod cors;
pub mod models;
pub mod routes;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::index, routes::world])
        .register(catchers![routes::not_found])
        .attach(cors::CorsFairing)
        .launch();
}
