use diesel;
use example::repositories::bike_db_repository;
use example::datasources::db;
use example::domains::bike_repo::BikeRepo;
use example::domains::bike::Bike;
use example::schema::bikes;
use diesel::RunQueryDsl;

mod common;


#[test]
fn test_diesel_bike_repository_create() {
    let pool = db::connect();
    let repo = bike_db_repository::DieselBikeRepository::new(pool);

    let description = Some("description".to_string());
    let model = Some("model".to_string());

    let mut bike = Bike {
        id: None,
        description: description.clone(),
        model: model.clone(),
    };
    let result = repo.create(&mut bike);

    if let Err(e) = &result {
        assert!(false, "error {}", e);
    }

    let bike = result.unwrap();
    let conn = db::connect().get().unwrap();
    diesel::delete(bikes::table).execute(&conn);

    assert!(bike.id.unwrap() > 0);
    assert_eq!(bike.description, description);
    assert_eq!(bike.model, model);
}

#[test]
fn test_diesel_bike_repository_read_all() {
    // TODO
}

#[test]
fn test_diesel_bike_repository_update() {
    // TODO
}

#[test]
fn test_diesel_bike_repository_delete() {
    // TODO
}
