#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use crate::datasources::db;
use crate::handlers::rest;

mod domains;
mod schema;

mod configs;
mod datasources;
mod handlers;
mod managers;
mod repositories;

pub fn run() {
    rocket::ignite()
        .manage(db::connect())
        // .manage(config)
        .mount("/", routes![rest::index])
        .mount("/bike", routes![rest::create, rest::update, rest::delete])
        .mount("/bikes", routes![rest::read])
        .launch();
}
