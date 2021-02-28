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
use rocket::Response;
use rocket::http::{Header, Status, ContentType};
use rocket::fairing::{Kind, Info, Fairing};

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

pub struct CORS();

impl Fairing for CORS {
  fn info(&self) -> Info {
    Info {
      name: "Add CORS headers to requests",
      kind: Kind::Response
    }
  }

  fn on_response(&self, _request: &Request, response: &mut Response) {
    response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
    response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    response.set_status(Status::Ok);
  }
}

fn main() {
  routes::fuel(rocket::ignite())
    .attach(CORS())
    .attach(CoolDb::fairing())
    .register(catchers![catch_not])
    .mount("/", routes![status])
    .launch();
}
