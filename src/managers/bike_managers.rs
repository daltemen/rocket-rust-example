use std::marker::Copy;
use crate::domains::bike::Bike;
use crate::domains::bike_repo::BikeRepo;
use crate::managers::manager_errors;
use crate::managers::manager_errors::BikesManagerError;
use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait BikeManager {
    fn create(&self, bike_in: BikeIn) -> Result<BikeOut, manager_errors::BikesManagerError>;
    fn read_all(&self) -> Result<Vec<BikeOut>, manager_errors::BikesManagerError>;
    fn update(
        &self,
        id: i32,
        bike_in: BikeIn,
    ) -> Result<BikeOut, manager_errors::BikesManagerError>;
    fn delete(&self, id: i32) -> Result<bool, manager_errors::BikesManagerError>;
}

pub struct BikeIn {
    pub description: Option<String>,
    pub model: Option<String>,
}

pub struct BikeOut {
    pub id: Option<i32>,
    pub description: Option<String>,
    pub model: Option<String>,
}

pub struct BikeCrudManager<T>
where
    T: BikeRepo,
{
    pub repo: T,
}

impl<T> BikeCrudManager<T>
where
    T: BikeRepo,
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

impl<T> BikeManager for BikeCrudManager<T>
where
    T: BikeRepo,
{
    fn create(&self, bike_in: BikeIn) -> Result<BikeOut, BikesManagerError> {
        let mut bike = Bike {
            id: None,
            description: bike_in.description,
            model: bike_in.model,
        };

        let result = self.repo.create(&mut bike);

        if let Err(e) = result {
            // TODO: log
            println!("Error: {}", e);
            return Err(BikesManagerError::RepositoryError(
                "Error creating bike".to_string(),
            ));
        }

        let bike = result.unwrap();

        let bike_out = BikeOut {
            id: bike.id.clone(),
            description: bike.description.clone(),
            model: bike.model.clone(),
        };

        Ok(bike_out)
    }

    fn read_all(&self) -> Result<Vec<BikeOut>, BikesManagerError> {
        let result = self.repo.read_all();
        if let Err(e) = result {
            // TODO: log
            println!("Error: {}", e);
            return Err(BikesManagerError::RepositoryError(
                "Error reading bike".to_string(),
            ));
        }

        let bikes = result.unwrap();
        let outs = bikes
            .iter()
            .map(|b| BikeOut {
                id: b.id.clone(),
                description: b.description.clone(),
                model: b.model.clone(),
            })
            .collect();
        Ok(outs)
    }

    fn update(&self, id: i32, bike_in: BikeIn) -> Result<BikeOut, BikesManagerError> {
        let bike = Bike {
            id: None,
            description: bike_in.description,
            model: bike_in.model,
        };
        let result = self.repo.update(id, &bike);
        if let Err(e) = result {
            // TODO: log
            println!("Error: {}", e);
            return Err(BikesManagerError::RepositoryError(
                "Error updating bike".to_string(),
            ));
        }

        let bike = result.unwrap();

        let bike_out = BikeOut {
            id: bike.id.clone(),
            description: bike.description.clone(),
            model: bike.model.clone(),
        };

        Ok(bike_out)
    }

    fn delete(&self, id: i32) -> Result<bool, BikesManagerError> {
        let result = self.repo.delete(id);
        if let Err(e) = result {
            // TODO: log
            println!("Error: {}", e);
            return Err(BikesManagerError::RepositoryError(
                "Error updating bike".to_string(),
            ));
        }

        let deleted = result.unwrap();
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::bike_errors;
    use crate::repositories::mocks::{BikeRepoMock, BikeRepoMockError};

    fn get_manager() -> BikeCrudManager<BikeRepoMock> {
        let repo_mock = BikeRepoMock::new();
        let manager = BikeCrudManager::new(repo_mock);

        manager
    }

    #[test]
    fn test_bike_crud_manager_create() {
        let manager = get_manager();

        let description = "description".to_string();
        let model = "model".to_string();

        let bike_in = BikeIn {
            description: Some(description.clone()),
            model: Some(model.clone()),
        };

        let bike_out = manager.create(bike_in).unwrap();

        assert_eq!(bike_out.description.unwrap(), description);
        assert_eq!(bike_out.model.unwrap(), model);
    }

    #[test]
    fn test_bike_crud_manager_read_all() {
        let manager = get_manager();

        let bike_out = manager.read_all().unwrap();

        assert_eq!(bike_out.len(), 2);
    }

    #[test]
    fn test_bike_crud_manager_update() {
        let manager = get_manager();

        let description = "description".to_string();
        let model = "model".to_string();

        let bike_in = BikeIn {
            description: Some(description.clone()),
            model: Some(model.clone()),
        };
        let bike_out = manager.update(1, bike_in).unwrap();

        assert_eq!(bike_out.description.unwrap(), description);
        assert_eq!(bike_out.model.unwrap(), model);
    }

    #[test]
    fn test_bike_crud_manager_delete() {
        let manager = get_manager();
        let bike_out = manager.delete(1).unwrap();

        assert!(bike_out);
    }

    fn get_manager_error() -> BikeCrudManager<BikeRepoMockError> {
        let repo_mock = BikeRepoMockError::new();
        let manager = BikeCrudManager::new(repo_mock);

        manager
    }

    #[test]
    fn test_bike_crud_manager_create_error() {
        let manager = get_manager_error();

        let description = "description".to_string();
        let model = "model".to_string();

        let bike_in = BikeIn {
            description: Some(description.clone()),
            model: Some(model.clone()),
        };

        let result = manager.create(bike_in).is_ok();

        assert_eq!(false, result);
    }

    #[test]
    fn test_bike_crud_manager_read_all_error() {
        let manager = get_manager_error();

        let result = manager.read_all().is_ok();

        assert_eq!(false, result);
    }

    #[test]
    fn test_bike_crud_manager_update_error() {
        let manager = get_manager_error();

        let description = "description".to_string();
        let model = "model".to_string();

        let bike_in = BikeIn {
            description: Some(description.clone()),
            model: Some(model.clone()),
        };
        let result = manager.update(1, bike_in).is_ok();

        assert_eq!(false, result);
    }

    #[test]
    fn test_bike_crud_manager_delete_error() {
        let manager = get_manager_error();
        let result = manager.delete(1).is_ok();

        assert_eq!(false, result);
    }
}
