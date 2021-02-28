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

  fn on_response(&self, request: &Request, response: &mut Response) {
    response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
    response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
  }
}

#[get("/")]
fn get_handler<'a>() -> Response<'a> {
  let mut res = Response::new();
  res.set_status(Status::new(200, "No Content"));
  res.adjoin_header(ContentType::Plain);
  res.adjoin_raw_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS");
  res.adjoin_raw_header("Access-Control-Allow-Origin", "*");
  res.adjoin_raw_header("Access-Control-Allow-Credentials", "true");
  res.adjoin_raw_header("Access-Control-Allow-Headers", "Content-Type");
  res
}

fn main() {
  routes::fuel(rocket::ignite())
    .attach(CoolDb::fairing())
    .register(catchers![catch_not])
    .mount("/", routes![status])
    .launch();
}
