use crate::datasources::db;
use crate::managers::bike_managers::{BikeCrudManager};
use crate::repositories::bike_db_repository::DieselBikeRepository;

pub struct ConfigStatus {
    pub manager: BikeCrudManager<DieselBikeRepository>,
}

impl ConfigStatus {
    pub fn new() -> Self {
        let pool = db::connect();
        let repo = DieselBikeRepository::new(pool);
        let manager = BikeCrudManager::new(repo);
        Self { manager }
    }
}
