#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rand::Rng;
use rocket::http::Status;
use chrono::Utc;
use rocket_prometheus::PrometheusMetrics;
use rocket::response::status::BadRequest;
use rocket::response::status;
use log::{info};


#[get("/version")] 
fn version() -> status::Accepted<String> {
    info!("Calling /version endpoint");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    info!("Version found in file: {}", VERSION);
    let now = Utc::now();

    status::Accepted(Some(format!("version {} @ {}", VERSION, now)))
}

#[get("/")]
fn index() -> status::Accepted<String> {
    info!("Calling /");
    status::Accepted(Some(format!("Hello, world!")))
}

#[get("/slow")] 
fn slow() -> status::Accepted<String> {
    use std::{thread, time};

    let mut rng = rand::thread_rng();
    let delay: u64 = rng.gen_range(0..20);

    println!("{}", delay);

    let ten_millis = time::Duration::from_secs(delay);

    thread::sleep(ten_millis);
    status::Accepted(Some(format!("Slow response!")))
}


#[get("/fail")]
fn fail() -> Status {
    info!("Calling /fail");
    create_error_status()
}

#[post("/vote/<color>")]
fn vote(color: Option<String>) -> Result<String, BadRequest<String>> {
    match color {
        Some(color_string) => {
            let voted_string = format!("You voted: {}", color_string);
            match color_string.as_str() {
                "green" => Ok(voted_string),
                "red" => Ok(voted_string),
                "yello" => Ok(voted_string),
                _ =>  Err(BadRequest(Some(String::from("Invalid choice!"))))
            }
        },
        _ => Err(BadRequest(Some(String::from("You did not vote"))))
    }

}


fn main() {
    env_logger::init();

    let prometheus = PrometheusMetrics::new();
    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/", routes![index, version, slow, fail, vote]).mount("/metrics", prometheus).launch();
}

fn create_error_status() -> Status {
    let mut rng = rand::thread_rng();
    let choice: u64 = rng.gen_range(0..11);
    match choice {
        0 => Status::InternalServerError,
        1 => Status::NotImplemented,
        2 => Status::BadGateway,
        3 => Status::ServiceUnavailable,
        4 => Status::GatewayTimeout,
        5 => Status::HttpVersionNotSupported,
        6 => Status::VariantAlsoNegotiates,
        7 => Status::InsufficientStorage,
        8 => Status::LoopDetected,
        9 => Status::NotExtended,
        10 => Status::NetworkAuthenticationRequired,
        _ => Status::InternalServerError
    }
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_verify_create_error_status_returns_error_code() {
        let result = create_error_status();
        assert!(result.code >= 500 && result.code <= 511)
    }

    #[test]
    fn test_vote_has_valid_color_green_returns_ok(){
        let result = vote(Some(String::from("green")));

        assert!(result.is_ok());
    }

    #[test]
    fn test_vote_has_valid_color_red_returns_ok(){
        let result = vote(Some(String::from("red")));

        assert!(result.is_ok());
    }

    #[test]
    fn test_vote_has_invalid_color_returns_err(){
        let result = vote(Some(String::from("yes")));

        assert!(result.is_err());
    }
}




