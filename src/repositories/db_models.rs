use crate::schema::bikes;

#[table_name = "bikes"]
#[derive(AsChangeset, Queryable, Insertable)]
pub struct BikeDB {
    pub id: i32,
    pub description: Option<String>,
    pub model: Option<String>,
}
