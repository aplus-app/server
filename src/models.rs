use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "post"]
pub struct Post {
	pub id: i32,
	pub category: String,
	pub user_name: String,
	pub user_id: String,
	pub title: String,
	pub body: String
}

impl Post {
	pub fn new<'a>(
		category: &'a str,
		user_name: &'a str,
		user_id: &'a str,
		title: &'a str,
		body: &'a str,
		conn: &PgConnection
	) -> Post {
		use super::schema::post;

		let u = InsertablePost {
			category,
			user_name,
			user_id,
			title,
			body
		};

		diesel::insert_into(post::table)
			.values(&u)
			.get_result(conn)
			.expect("Error saving post")
	}
}

use super::schema::post;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "post"]
pub struct InsertablePost<'a> {
	pub category: &'a str,
	pub user_name: &'a str,
	pub user_id: &'a str,
	pub title: &'a str,
	pub body: &'a str
}