use crate::managers::bike_managers::BikeCrudManager;
use crate::repositories::bike_db_repository::DieselBikeRepository;
use diesel::MysqlConnection;

pub fn new_manager(connection: &MysqlConnection) -> BikeCrudManager<DieselBikeRepository> {
    let repo = DieselBikeRepository::new(connection);
    let manager = BikeCrudManager::new(repo);

    manager
}

// use rocket::http::Status;
// use diesel::{MysqlConnection, r2d2};
// use crate::managers::bike_managers::{BikeManager, BikeCrudManager};
// use crate::domains::bike_repo::BikeRepo;
// use crate::repositories::bike_db_repository::DieselBikeRepository;
// use diesel::r2d2::ConnectionManager;
// use std::ops::Deref;
// use rocket::request::FromRequest;
// use rocket::{Request, request, State, Outcome};
// use crate::datasources::db::Pool;
//
// pub struct ConfigStatus {
//     pub pool: r2d2::PooledConnection<ConnectionManager<MysqlConnection>>,
//     pub manager: BikeCrudManager<DieselBikeRepository>,
// }
//
// impl ConfigStatus {
//     fn new(pool: r2d2::PooledConnection<ConnectionManager<MysqlConnection>>, manager: BikeCrudManager<DieselBikeRepository>) -> Self {
//         Self {
//             pool,
//             manager,
//         }
//     }
// }
//
// impl<'a, 'r> FromRequest<'a, 'r> for ConfigStatus {
//     type Error = ();
//
//     fn from_request(request: &'a Request<'r>) -> request::Outcome<ConfigStatus, ()> {
//         let pool = request.guard::<State<Pool>>()?;
//         match pool.get() {
//             Ok(conn) => {
//                 let repo = DieselBikeRepository::new();
//                 let manager = BikeCrudManager::new(repo);
//
//                 let config_status = ConfigStatus::new(conn, manager);
//                 Outcome::Success(config_status)
//             }
//             Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
//         }
//     }
// }
