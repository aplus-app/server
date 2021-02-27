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
