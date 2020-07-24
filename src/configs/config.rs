use crate::datasources::db;
use crate::domains::bike_repo::BikeRepo;
use crate::managers::bike_managers::{BikeCrudManager, BikeManager};
use crate::repositories::bike_db_repository::DieselBikeRepository;

pub struct ConfigStatus {
    pub manager: Box<dyn BikeManager + Send + Sync + 'static>,
}

impl ConfigStatus {
    pub fn new(manager: Box<dyn BikeManager + Send + Sync + 'static>) -> Self {
        Self { manager }
    }
}
