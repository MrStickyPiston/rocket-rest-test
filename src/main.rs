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

#[derive(Deserialize)]
pub struct CreateRoot {
	pub root: String,
	pub is_regular: bool,
	pub conjugation_group: String,
    	pub gender: i64,
   	pub metadata: serde_json::Value,
}

#[get("/")]
async fn get_roots(state: &State<AppState>) -> Option<Json<Response<Vec<NounRootsTable>>>> {
    sqlx::query_as("SELECT * FROM noun_roots_table")
        .fetch_all(&state.pool).await
	.map(|v| {Response{value: v}})
        .map(Json)
        .ok()
}

#[post("/", data="<root>", format = "json")]
async fn create_root(state: &State<AppState>, root: Json<CreateRoot>) -> Json<Response<String>> {
    sqlx::query("INSERT INTO noun_roots_table (root, is_regular, conjugation_group, gender, metadata) VALUES ($1, $2, $3, $4, $5);")
	.bind(&root.root)                // Bind the root field
	.bind(root.is_regular)           // Bind the is_regular field
    	.bind(&root.conjugation_group)   // Bind the conjugation_group field
    	.bind(root.gender)                // Bind the gender field
    	.bind(&root.metadata)             // Bind the metadata field

        .fetch_one(&state.pool).await
        .ok();

	Json(Response{value: "Root created".to_string()})
}

#[get("/<id>")]
async fn get_root(state: &State<AppState>, id: i64) -> Option<Json<Response<NounRootsTable>>> {
    sqlx::query_as("SELECT * FROM noun_roots_table WHERE id = $1;").bind(id)
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
	sqlx::migrate!()
        	.run(&pool)
        	.await
        	.expect("Failed to run migrations");

	let state = AppState { pool };
 
	let rocket = rocket::build()
		.mount("/roots/", routes![get_roots, get_root, create_root])
		.manage(state);
	
	Ok(rocket.into())
}

