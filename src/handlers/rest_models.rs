#[derive(Serialize, Deserialize)]
pub struct BikeRequest {
    pub description: String,
    pub model: String,
}

#[derive(Serialize, Deserialize)]
pub struct BikeResponse {
    pub id: i32,
    pub description: String,
    pub model: String,
}
