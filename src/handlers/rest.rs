use rocket_contrib::json::{Json, JsonValue};
use crate::handlers::rest_models::{BikeRequest, BikeResponse};
// use crate::db;
use crate::configs::config;
use crate::repositories::bike_db_repository;
use crate::domains::bike::Bike;
use crate::domains::bike_repo::BikeRepo;
use crate::managers::bike_managers::{BikeManager, BikeIn};
use crate::datasources::db;
use rocket::State;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<bike>")]
pub fn create(bike: Json<BikeRequest>, connection: db::Connection) -> Json<BikeResponse> {
    let manager = config::new_manager(&connection);

    let bike = bike.into_inner();
    let bike = BikeIn {
        description: Some(bike.description),
        model: Some(bike.model),
    };

    let out = manager.create(bike).unwrap();

    Json(BikeResponse {
        id: out.id.as_ref().unwrap().clone(),
        description: out.description.as_ref().unwrap().clone(),
        model: out.model.as_ref().unwrap().clone(),
    })
}

#[get("/")]
pub fn read(connection: db::Connection) -> Json<JsonValue> {
    let manager = config::new_manager(&connection);

    let bikes = manager.read_all().unwrap();

    let response: Vec<BikeResponse> = bikes.iter()
        .map(|b| BikeResponse {
            id: b.id.as_ref().unwrap().clone(),
            description: b.description.as_ref().unwrap().clone(),
            model: b.model.as_ref().unwrap().clone(),
        })
        .collect();

    Json(json!(response))
}

#[put("/<id>", data = "<bike>")]
pub fn update(id: i32, bike: Json<BikeRequest>, connection: db::Connection) -> Json<BikeResponse> {
    let manager = config::new_manager(&connection);
    let bike = bike.into_inner();

    let bike = BikeIn {
        description: Some(bike.description),
        model: Some(bike.model),
    };
    // TODO: handle error
    let out = manager.update(id, bike).unwrap();

    Json(BikeResponse {
        id: id,
        description: out.description.as_ref().unwrap().clone(),
        model: out.model.as_ref().unwrap().clone(),
    })
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: db::Connection) -> Json<JsonValue> {
    let manager = config::new_manager(&connection);

    let result = manager.delete(id).unwrap();
    if result {
        Json(json!({"status": "ok"}))
    } else {
        Json(json!({"status": "failed"}))
    }
}
