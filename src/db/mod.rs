use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn estabilish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connectin to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Erro inserting new task");
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}
