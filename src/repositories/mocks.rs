use crate::domains::bike_repo;
use crate::domains::bike_errors::BikesError;
use crate::domains::bike::Bike;

pub struct BikeRepoMock {}

impl BikeRepoMock {
    pub fn new() -> Self {
        Self {}
    }
}

impl bike_repo::BikeRepo for BikeRepoMock {
    fn create<'a, 'b>(&'a self, bike: &'b mut Bike) -> Result<&'b Bike, BikesError> {
        bike.id = Some(1);
        Ok(bike)
    }

    fn read_all(&self) -> Result<Vec<Bike>, BikesError> {
        let bike1 = Bike {
            id: Some(1),
            description: Some("description".to_string()),
            model: Some("model".to_string()),
        };

        let bike2 = Bike {
            id: Some(2),
            description: Some("description".to_string()),
            model: Some("model".to_string()),
        };

        let vec1 = vec![bike1, bike2];
        Ok(vec1)
    }

    fn update<'a, 'b>(&'a self, _id: i32, bike: &'b Bike) -> Result<&'b Bike, BikesError> {
        Ok(bike)
    }

    fn delete(&self, _id: i32) -> Result<bool, BikesError> {
        Ok(true)
    }
}