#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use crate::configs::config;
use crate::datasources::db;
use crate::handlers::rest;
use crate::managers::bike_managers::BikeCrudManager;
use crate::repositories::bike_db_repository::DieselBikeRepository;

mod domains;
mod schema;

mod configs;
mod datasources;
mod handlers;
mod managers;
pub mod repositories;

pub fn run() {
    let pool = db::connect();
    let repo = DieselBikeRepository::new(pool);
    let manager = BikeCrudManager::new(repo);

    rocket::ignite()
        .manage(config::ConfigStatus::new(Box::new(manager)))
        .mount("/", routes![rest::index])
        .mount("/bike", routes![rest::create, rest::update, rest::delete])
        .mount("/bikes", routes![rest::read])
        .launch();
}
