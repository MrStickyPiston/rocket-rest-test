#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::serde_json;
use rocket_db_pools::{Database};

use rocket_db_pools::Connection;
use sqlx;

#[derive(Serialize, Debug)]
struct Response<T> {
	value: T
}

#[derive(sqlx::FromRow, Serialize)]
pub struct NounRootsTable {
    pub id: i64,
    pub root: String,
    pub is_regular: bool,
    pub conjugation_group: String,
    pub gender: i64,
    pub metadata: serde_json::Value,
}


#[get("/")]
fn index() -> Json<Response<String>> {
    Json(Response{value: "test".to_string()})
}

#[derive(Database)]
#[database("db")]
struct Db(sqlx::SqlitePool);

#[get("/")]
async fn get_roots(mut db: Connection<Db>) -> Option<Json<Response<Vec<NounRootsTable>>>> {
    sqlx::query_as("SELECT * FROM noun_roots_table")
        .fetch_all(&mut **db).await
	.map(|v| {Response{value: v}})
        .map(Json)
        .ok()
}

#[get("/<id>")]
async fn get_root(mut db: Connection<Db>, id: i64) -> Option<Json<Response<NounRootsTable>>> {
    sqlx::query_as("SELECT * FROM noun_roots_table WHERE id = ?").bind(id)
        .fetch_one(&mut **db).await
	.map(|v| {Response{value: v}})
	.map(Json)
	.ok()
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
	.mount("/", routes![index])
	.mount("/roots/", routes![get_roots, get_root])
	.attach(Db::init());
	    Ok(rocket.into())
}

