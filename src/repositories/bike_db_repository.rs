use crate::domains::bike::Bike;
use crate::domains::bike_errors;
use crate::domains::bike_errors::BikesError;
use crate::domains::bike_repo;
use crate::repositories::db_models::BikeDB;
use crate::schema::bikes;
use crate::schema::bikes::dsl::*;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub struct DieselBikeRepository<'z> {
    connection: &'z MysqlConnection,
}

impl<'z> DieselBikeRepository<'z> {
    pub fn new(connection: &'z MysqlConnection) -> Self {
        Self { connection }
    }
}

impl<'z> bike_repo::BikeRepo for DieselBikeRepository<'z> {
    fn create<'a, 'b>(&'a self, bike: &'b mut Bike) -> Result<&'b Bike, BikesError> {
        let query_result = diesel::insert_into(bikes::table)
            .values((description.eq(&bike.description), model.eq(&bike.model)))
            .execute(self.connection);

        if let Err(e) = query_result {
            // TODO: log
            println!("Error: {}", e);
            return Err(bike_errors::BikesError::DBError(
                "Error creating bikes".to_string(),
            ));
        }
        let bike_db_result: Result<BikeDB, diesel::result::Error> =
            bikes::table.order(id.desc()).first(self.connection);

        if let Err(e) = bike_db_result {
            // TODO: log
            println!("Error: {}", e);
            return Err(bike_errors::BikesError::DBError(
                "Error getting bikes".to_string(),
            ));
        }
        let bike_db = bike_db_result.unwrap();
        bike.id = Some(bike_db.id);

        Ok(bike)
    }

    fn read_all(&self) -> Result<Vec<Bike>, BikesError> {
        let query_result: Result<Vec<BikeDB>, diesel::result::Error> = bikes::table
            .order(bikes::id.asc())
            .load::<BikeDB>(self.connection);

        if let Err(e) = query_result {
            // TODO: log
            println!("Error: {}", e);
            return Err(bike_errors::BikesError::DBError(
                "Error reading all bikes".to_string(),
            ));
        }

        let bikes_vec = query_result.unwrap();
        let result = bikes_vec
            .iter()
            .map(|b| Bike {
                id: Some(b.id),
                description: b.description.clone(),
                model: b.model.clone(),
            })
            .collect();

        Ok(result)
    }

    fn update<'a, 'b>(&'a self, id_bike: i32, bike: &'b Bike) -> Result<&'b Bike, BikesError> {
        let query_result = diesel::update(bikes::table.find(id_bike))
            .set((description.eq(&bike.description), model.eq(&bike.model)))
            .execute(self.connection);

        if let Err(e) = query_result {
            // TODO: log
            println!("Error: {}", e);
            return Err(bike_errors::BikesError::DBError(
                "Error updating bikes".to_string(),
            ));
        }

        Ok(bike)
    }

    fn delete(&self, id_bike: i32) -> Result<bool, bike_errors::BikesError> {
        let query_result = diesel::delete(bikes::table.find(id_bike)).execute(self.connection);
        if let Err(e) = query_result {
            // TODO: log
            println!("Error: {}", e);
            return Err(bike_errors::BikesError::DBError(
                "Error deleting bikes".to_string(),
            ));
        }

        Ok(true)
    }
}
