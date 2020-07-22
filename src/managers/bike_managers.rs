use crate::managers::manager_errors;
use crate::domains::bike_repo::BikeRepo;
use crate::managers::manager_errors::BikesManagerError;
use crate::domains::bike::Bike;

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
        T: BikeRepo
{
    pub repo: T,
}

impl<T> BikeCrudManager<T> where
    T: BikeRepo
{
    pub fn new(repo: T) -> Self {
        Self {
            repo
        }
    }
}

impl<T> BikeManager for BikeCrudManager<T> where
    T: BikeRepo
{
    fn create(&self, bike_in: BikeIn) -> Result<BikeOut, BikesManagerError> {
        let mut bike = Bike {
            id: None,
            description: bike_in.description,
            model: bike_in.model,
        };

        // TODO: handle error
        let bike = self.repo.create(&mut bike).unwrap();

        let bike_out = BikeOut {
            id: bike.id.clone(),
            description: bike.description.clone(),
            model: bike.model.clone(),
        };

        Ok(bike_out)
    }

    fn read_all(&self) -> Result<Vec<BikeOut>, BikesManagerError> {
        // TODO: handle error
        let bikes = self.repo.read_all().unwrap();
        let outs = bikes.iter()
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

        // TODO: handle error
        let bike = self.repo.update(id, &bike).unwrap();
        let bike_out = BikeOut {
            id: bike.id.clone(),
            description: bike.description.clone(),
            model: bike.model.clone(),
        };

        Ok(bike_out)
    }

    fn delete(&self, id: i32) -> Result<bool, BikesManagerError> {
        // TODO: handle error
        let deleted = self.repo.delete(id).unwrap();
        Ok(deleted)
    }
}
