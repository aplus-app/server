use crate::models::{Comment, Post};
use crate::CoolDb;
use rocket::Rocket;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

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
        0,
        &conn.0,
    ))
}

#[derive(Serialize, Deserialize)]
struct DeletePostJson {
    id: i32,
}

#[post("/delete-post", format = "json", data = "<input>")]
fn delete_post(input: Json<DeletePostJson>, conn: CoolDb) -> Json<usize> {
    Json(Post::delete(input.id, &conn.0))
}

// ===================================== COMMENTS =====================================

#[derive(Serialize, Deserialize)]
struct CreateCommentJson {
    post_id: String,
    body: String,
}

#[post("/create-comment", format = "json", data = "<input>")]
fn create_comment(input: Json<CreateCommentJson>, conn: CoolDb) -> Json<Comment> {
    Json(Comment::new(&input.post_id, &input.body, &conn.0))
}

#[derive(Serialize, Deserialize)]
struct DeleteCommentJson {
    id: i32,
}

#[post("/delete-comment", format = "json", data = "<input>")]
fn delete_comment(input: Json<DeleteCommentJson>, conn: CoolDb) -> Json<usize> {
    Json(Comment::delete(input.id, &conn.0))
}

#[derive(Serialize, Deserialize)]
struct HeartPostJson {
    id: i32,
}

#[post("/add-heart", format = "json", data = "<input>")]
fn add_heart(input: Json<HeartPostJson>, conn: CoolDb) -> Json<Post> {
    Json(Post::heart(input.id, &conn.0))
}

#[post("/remove-heart", format = "json", data = "<input>")]
fn remove_heart(input: Json<HeartPostJson>, conn: CoolDb) -> Json<Post> {
    Json(Post::unheart(input.id, &conn.0))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/",
        routes![
      create_post,
      delete_post,
      create_comment,
      delete_comment,
      add_heart,
      remove_heart,
    ],
    )
}
