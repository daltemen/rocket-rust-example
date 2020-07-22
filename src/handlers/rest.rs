use rocket_contrib::json::{Json, JsonValue};
use rocket::http::{Status};
use crate::handlers::rest_models::{BikeRequest, BikeResponse, ApiResponse};
// use crate::db;
use crate::configs::config;
use crate::managers::bike_managers::{BikeManager, BikeIn};
use crate::datasources::db;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<bike>")]
pub fn create(bike: Json<BikeRequest>, connection: db::Connection) -> ApiResponse {
    let manager = config::new_manager(&connection);

    let bike = bike.into_inner();
    let bike = BikeIn {
        description: bike.description,
        model: bike.model,
    };

    let out = manager.create(bike).unwrap();

    let response = BikeResponse {
        id: out.id.as_ref().unwrap().clone(),
        description: out.description,
        model: out.model,
    };

    ApiResponse {
        json: json!(response),
        status: Status::Created,
    }
}

#[get("/")]
pub fn read(connection: db::Connection) -> Json<JsonValue> {
    let manager = config::new_manager(&connection);

    let bikes = manager.read_all().unwrap();

    let response: Vec<BikeResponse> = bikes.iter()
        .map(|b| BikeResponse {
            id: b.id.as_ref().unwrap().clone(),
            description: b.description.clone(),
            model: b.model.clone(),
        })
        .collect();

    Json(json!(response))
}

#[put("/<id>", data = "<bike>")]
pub fn update(id: i32, bike: Json<BikeRequest>, connection: db::Connection) -> Json<BikeResponse> {
    let manager = config::new_manager(&connection);
    let bike = bike.into_inner();

    let bike = BikeIn {
        description: bike.description,
        model: bike.model,
    };
    // TODO: handle error
    let out = manager.update(id, bike).unwrap();

    Json(BikeResponse {
        id: id,
        description: out.description.clone(),
        model: out.model.clone(),
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
