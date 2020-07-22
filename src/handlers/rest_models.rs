use rocket::http::{ContentType, Status};
use rocket_contrib::json::JsonValue;
use rocket::response::Responder;
use rocket::{Request, response, Response};

#[derive(Serialize, Deserialize)]
pub struct BikeRequest {
    pub description: Option<String>,
    pub model: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BikeResponse {
    pub id: i32,
    pub description: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
