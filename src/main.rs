#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_imports)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod routes;
pub mod schema;
use diesel::prelude::*;
use rocket::request::Request;
use rocket_contrib::databases::diesel::PgConnection;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[database("cooldb")]
struct CoolDb(PgConnection);

#[get("/ok")]
fn status(_conn: CoolDb) -> &'static str {
  "OK"
}

#[catch(503)]
fn catch_not(_req: &Request) -> &'static str {
  "Service is not available. (Is the database up?)"
}

fn main() {
  routes::fuel(rocket::ignite())
    .attach(CoolDb::fairing())
    .register(catchers![catch_not])
    .mount("/", routes![status])
    .launch();
}
