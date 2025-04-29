#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::serde_json;
use rocket::State;

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

#[get("/")]
async fn get_roots(state: &State<AppState>) -> Option<Json<Response<Vec<NounRootsTable>>>> {
    sqlx::query_as("SELECT * FROM noun_roots_table")
        .fetch_all(&state.pool).await
	.map(|v| {Response{value: v}})
        .map(Json)
        .ok()
}

#[get("/<id>")]
async fn get_root(state: &State<AppState>, id: i64) -> Option<Json<Response<NounRootsTable>>> {
    sqlx::query_as("SELECT * FROM noun_roots_table WHERE id = ?").bind(id)
        .fetch_one(&state.pool).await
	.map(|v| {Response{value: v}})
	.map(Json)
	.ok()
}

struct AppState {
	pool: sqlx::PgPool
}

#[shuttle_runtime::main]
async fn rocket(
	#[shuttle_shared_db::Postgres(
		local_uri = "postgresql://user@127.0.0.1:5432/rest-test"
	)]
	pool: sqlx::PgPool
) -> shuttle_rocket::ShuttleRocket {
	let state = AppState { pool };
 
	let rocket = rocket::build()
		.mount("/", routes![index])
		.mount("/roots/", routes![get_roots, get_root])
		.manage(state);
	
	Ok(rocket.into())
}

