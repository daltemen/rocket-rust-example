use crate::configs::config;
use crate::handlers::rest_models::{ApiResponse, BikeRequest, BikeResponse};
use crate::managers::bike_managers::{BikeIn, BikeManager};
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<bike>")]
pub fn create(bike: Json<BikeRequest>, config: State<config::ConfigStatus>) -> ApiResponse {
    let manager = &config.manager;

    let bike = bike.into_inner();
    let bike = BikeIn {
        description: bike.description,
        model: bike.model,
    };

    // TODO: handle error
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
pub fn read(config: State<config::ConfigStatus>) -> Json<JsonValue> {
    let manager = &config.manager;

    // TODO: handle error
    let bikes = manager.read_all().unwrap();

    let response: Vec<BikeResponse> = bikes
        .iter()
        .map(|b| BikeResponse {
            id: b.id.as_ref().unwrap().clone(),
            description: b.description.clone(),
            model: b.model.clone(),
        })
        .collect();

    Json(json!(response))
}

#[put("/<id>", data = "<bike>")]
pub fn update(
    id: i32,
    bike: Json<BikeRequest>,
    config: State<config::ConfigStatus>,
) -> Json<BikeResponse> {
    let manager = &config.manager;
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
pub fn delete(id: i32, config: State<config::ConfigStatus>) -> Json<JsonValue> {
    let manager = &config.manager;

    // TODO: handle error
    let result = manager.delete(id).unwrap();
    if result {
        Json(json!({"status": "ok"}))
    } else {
        Json(json!({"status": "failed"}))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::configs::config;
    use crate::configs::config::ConfigStatus;
    use crate::managers::bike_managers::*;
    use crate::managers::bike_managers;
    use mockall::predicate::*;
    use mockall::*;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    fn rocket(mock: bike_managers::MockBikeManager) -> rocket::Rocket {
        rocket::ignite()
            .manage(config::ConfigStatus::new(Box::new(mock)))
            .mount("/", routes![index])
            .mount("/bike", routes![create, update, delete])
            .mount("/bikes", routes![read])
    }

    #[test]
    fn test_rest_index() {
        let mut mock = MockBikeManager::new();
        let client = Client::new(rocket(mock)).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn test_rest_create() {
        let mut mock = MockBikeManager::new();

        mock.expect_create().returning(|_| Ok(BikeOut {
            id: Some(1),
            description: Some("description".into()),
            model: Some("model".into()),
        }));

        let client = Client::new(rocket(mock)).expect("valid rocket instance");
        let mut response = client
            .post("/bike")
            .header(ContentType::JSON)
            .body(
                r#"
                {
	"description": "description",
	"model": "model"
}
"#, )
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.body_string(), Some(r#"{"description":"description","id":1,"model":"model"}"#.into()));
    }

    #[test]
    fn test_rest_read() {
        let mut mock = MockBikeManager::new();

        mock.expect_read_all().returning(|| Ok(vec![BikeOut {
            id: Some(1),
            description: Some("description".into()),
            model: Some("model".into()),
        }]));

        let client = Client::new(rocket(mock)).expect("valid rocket instance");
        let mut response = client.get("/bikes").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"[{"description":"description","id":1,"model":"model"}]"#.into()));
    }

    #[test]
    fn test_rest_update() {
        let mut mock = MockBikeManager::new();

        mock.expect_update().returning(|_, _| Ok(BikeOut {
            id: None,
            description: Some("description".into()),
            model: Some("model".into()),
        }));

        let client = Client::new(rocket(mock)).expect("valid rocket instance");
        let mut response = client
            .put("/bike/1")
            .header(ContentType::JSON)
            .body(r#"
                {
	"description": "description",
	"model": "model"
}
"#, )
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"id":1,"description":"description","model":"model"}"#.into()));
    }

    #[test]
    fn test_rest_delete() {
        let mut mock = MockBikeManager::new();

        mock.expect_delete().returning(|_| Ok(true));

        let client = Client::new(rocket(mock)).expect("valid rocket instance");
        let mut response = client.delete("/bike/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(r#"{"status":"ok"}"#.into()));
    }
}
