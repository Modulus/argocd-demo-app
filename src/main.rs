#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::Status;
use chrono::Utc;

#[get("/version")] 
fn version() -> String {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let now = Utc::now();

    format!("version {} @ {}", VERSION, now)
}

#[get("/")]
fn index() -> String {
    format!("Hello, world!")
}

#[get("/slow")] 
fn slow() -> String {
    use std::{thread, time};

    let ten_millis = time::Duration::from_secs(3);
    // let now = time::Instant::now();

    thread::sleep(ten_millis);
    format!("Slow response!")
}

#[get("/fail1")]
fn fail1() -> Status {
    Status::NotAcceptable
}

#[get("/fail2")]
fn fail2() -> Status {
    Status::InternalServerError
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, version, slow, fail1, fail2]).launch();
}