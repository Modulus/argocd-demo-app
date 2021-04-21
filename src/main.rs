#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rand::Rng;
use rocket::http::Status;
use chrono::Utc;
use rocket_prometheus::PrometheusMetrics;
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

fn main() {
    env_logger::init();

    let prometheus = PrometheusMetrics::new();
    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/", routes![index, version, slow, fail]).mount("/metrics", prometheus).launch();
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
}




