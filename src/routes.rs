use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use crate::CoolDb;
use crate::models::Post;
use std::borrow::Borrow;
use rocket::Rocket;
use rocket::response::content;

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
struct RemovePostJson {
	post_id: i32,
}

#[post("/remove-post", format = "json", data = "<input>")]
fn remove_post(input: Json<RemovePostJson>, conn: CoolDb) -> Json<&'static str> {
	use crate::schema::post::dsl::*;
	use diesel::prelude::*;

	let count = diesel::delete(post.filter(id.eq(input.id))).execute(&conn)?;
    if count == 0 {
		return Json("{ 'status': 'false', 'error': 'The specified post was not found.' }")
	}
	Json("{ 'status': 'true' }")
}

pub fn fuel(rocket: Rocket) -> Rocket {
	rocket.mount("/", routes![create_post])
}