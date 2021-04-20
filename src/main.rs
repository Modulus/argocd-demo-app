#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::Status;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
        .mount("/", routes![index, fail1, fail2]).launch();
}