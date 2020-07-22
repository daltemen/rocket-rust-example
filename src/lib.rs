#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use crate::handlers::rest;
use crate::datasources::db;

mod domains;
mod schema;

mod managers;
mod repositories;
mod handlers;
mod datasources;
mod configs;


pub fn run() {
    rocket::ignite()
        .manage(db::connect())
        // .manage(config)
        .mount("/", routes![rest::index])
        .mount("/bike", routes![rest::create, rest::update, rest::delete])
        .mount("/bikes", routes![rest::read])
        .launch();
}
