use rocket_contrib::databases::mongodb;

#[database("users")]
pub struct MongoDbConn(mongodb::db::Database);