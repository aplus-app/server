use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use crate::CoolDb;
use crate::models::Post;
use std::borrow::Borrow;
use rocket::Rocket;

#[derive(Serialize, Deserialize)]
struct CreatePostJson {
	category: String,
	user_id: String,
	user_name: String,
	title: String,
	body: String,
}

#[post("/create-post", format = "json", data = "<input>")]
fn create_post(input: Json<CreatePostJson>, conn: CoolDb) -> Json<Post> {
	Json(Post::new(
		&input.category,
		&input.user_name,
		&input.user_id,
		&input.title,
		&input.body,
		&conn.0
	))
}

#[derive(Serialize, Deserialize)]
struct DeletePostJson {
	id: i32
}

#[post("/delete-post", format = "json" data = "<input>")]
fn delete_post(input: Json<DeletePostJson>, conn: CoolDb) -> Json<usize> {
	Json(Post::delete(input.id, &conn.0))
}

pub fn fuel(rocket: Rocket) -> Rocket {
	rocket.mount("/", routes![create_post])
}