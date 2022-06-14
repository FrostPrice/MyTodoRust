#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use mytodo::db::models::Task;
use mytodo::db::{estabilish_connection, query_task};
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>,
}

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    let conn = estabilish_connection();
    for task in query_task(&conn) {
        response.data.push(task);
    }

    Json(response)
}

fn main() {
    rocket::ignite().mount("/", routes![tasks_get]).launch();
}
