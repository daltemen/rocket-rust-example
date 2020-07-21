use crate::domains::bike::Bike;
use crate::domains::bike_errors;

pub trait BikeRepo {
    fn create<'a, 'b>(&'a self, bike: &'b mut Bike) -> Result<&'b Bike, bike_errors::BikesError>;
    fn read_all(&self) -> Result<Vec<Bike>, bike_errors::BikesError>;
    fn update<'a, 'b>(
        &'a self,
        id: i32,
        bike: &'b Bike,
    ) -> Result<&'b Bike, bike_errors::BikesError>;
    fn delete(&self, id: i32) -> Result<bool, bike_errors::BikesError>;
}
