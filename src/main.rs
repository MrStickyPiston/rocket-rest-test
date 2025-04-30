#[macro_use] extern crate rocket;

use rocket::serde::json::{Json, Value};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, FromRow, PgPool, Row, query_as};

use serde::Serialize;

#[derive(Serialize, Debug)]
struct Response<T> {
	value: T
}

#[derive(FromRow, Serialize)]
pub struct NounRootsTable {
    pub id: i64,
    pub root: String,
    pub is_regular: bool,
    pub conjugation_group: String,
    pub gender: i64,
    pub metadata: Value,
}


#[get("/")]
fn index() -> Json<Response<String>> {
    Json(Response{value: "test".to_string()})
}

#[derive(Database)]
#[database("db")]
struct Db(PgPool);

#[get("/")]
async fn get_roots(mut db: Connection<Db>) -> Option<Json<Response<Vec<NounRootsTable>>>> {
    sqlx::query_as("SELECT * FROM noun_roots_table")
        .fetch_all(&mut **db).await
	.map(|v| {Response{value: v}})
        .map(Json)
        .ok()
}

#[shuttle_runtime::main]
async fn rocket(

) -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
	.mount("/", routes![index])
	.mount("/roots/", routes![get_roots])
	.attach(Db::init());
	    Ok(rocket.into())
}

