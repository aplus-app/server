use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "post"]
pub struct Post {
  pub id: i32,
  pub category: String,
  pub user_name: String,
  pub user_id: String,
  pub title: String,
  pub body: String,
  pub hearts: i32,
}

impl Post {
  pub fn new<'a>(
    category: &'a str,
    user_name: &'a str,
    user_id: &'a str,
    title: &'a str,
    body: &'a str,
    hearts: i32,
    conn: &PgConnection,
  ) -> Post {
    use super::schema::post;

    let u = InsertablePost {
      category,
      user_name,
      user_id,
      title,
      body,
      hearts
    };

    diesel::insert_into(post::table)
      .values(&u)
      .get_result(conn)
      .expect("Error saving post")
  }

  pub fn delete(postid: i32, conn: &PgConnection) -> usize {
    use crate::schema::post::dsl::*;
    diesel::delete(post.filter(id.eq(postid)))
      .execute(conn)
      .expect("Error deleting")
  }

  pub fn heart(post_id: i32, conn: &PgConnection) -> Post {
    use crate::schema::post::dsl::*;
    diesel::update(post.filter(id.eq(post_id)))
        .set(hearts.eq(hearts.clone() + 1))
        .get_result(conn)
        .expect("Could not add heart to the post.")
  }

  pub fn unheart(post_id: i32, conn: &PgConnection) -> Post {
    use crate::schema::post::dsl::*;
    diesel::update(post.filter(id.eq(post_id)))
        .set(hearts.eq(hearts.clone() - 1))
        .get_result(conn)
        .expect("Could not add heart to the post.")
  }}

use super::schema::post;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "post"]
pub struct InsertablePost<'a> {
  pub category: &'a str,
  pub user_name: &'a str,
  pub user_id: &'a str,
  pub title: &'a str,
  pub body: &'a str,
  pub hearts: i32,
}

// ======================== COMMENT MODELS ==================================================

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[belongs_to(Post)]
#[table_name = "comment"]
pub struct Comment {
  pub id: i32,
  pub post_id: String,
  pub body: String,
}

impl Comment {
  pub fn new(post_id: &str, body: &str, conn: &PgConnection) -> Comment {
    use super::schema::comment;

    let u = InsertableComment {
      post_id,
      body,
    };

    diesel::insert_into(comment::table)
      .values(&u)
      .get_result(conn)
      .expect("Error saving comment")
  }

  pub fn delete(comment_id: i32, conn: &PgConnection) -> usize {
    use crate::schema::comment::dsl::*;
    diesel::delete(comment.filter(id.eq(comment_id)))
      .execute(conn)
      .expect("Error deleting comment.")
  }


}

use crate::schema::comment;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "comment"]
pub struct InsertableComment<'a> {
  pub post_id: &'a str,
  pub body: &'a str,
}
