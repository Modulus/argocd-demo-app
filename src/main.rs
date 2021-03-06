#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


use rand::Rng;
use rocket::http::Status;
use chrono::Utc;
use rocket_prometheus::PrometheusMetrics;
use rocket::response::status::BadRequest;
use rocket::response::status;
use log::{info, error};

use once_cell::sync::Lazy;
use rocket_prometheus::{
    prometheus::{opts, IntCounterVec},
};

static GREEN_VOTES_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("votes_green", "Count of green votes"), &["green"])
        .expect("Could not create votes_green counter")
});

static YELLOW_VOTES_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("votes_yellow", "Count of yellow votes"), &["yellow"])
        .expect("Could not create votes_yellow counter")
});

static RED_VOTES_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("votes_red", "Count of red votes"), &["red"])
        .expect("Could not create votes_red counter")
});





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
            info!("{}", voted_string);
            match color_string.as_str() {
                "green" => {
                    info!("Green vote registered!"); 
                    GREEN_VOTES_COUNTER.with_label_values(&[color_string.as_str()]).inc();
                    Ok(voted_string)
                }
                "red" => {
                    info!("Red vote registered!");
                    RED_VOTES_COUNTER.with_label_values(&[color_string.as_str()]).inc();
                    Ok(voted_string)
                }
                "yellow" => {
                    info!("Yellow vote registered!");
                    YELLOW_VOTES_COUNTER.with_label_values(&[color_string.as_str()]).inc();
                    Ok(voted_string)
                }
                _ =>  {
                    error!("Invalid color vote!");
                    Err(BadRequest(Some(String::from("Invalid choice!"))))
                }
            }
        },
        _ => {
            error!("No color in request path");
            Err(BadRequest(Some(String::from("You did not vote"))))}
    }

}

fn main() {
    env_logger::init();

    let prometheus = PrometheusMetrics::new();

    prometheus.registry()
        .register(Box::new(GREEN_VOTES_COUNTER.clone()))
        .expect("Failed to register green votes counter!");

    prometheus.registry()
        .register(Box::new(YELLOW_VOTES_COUNTER.clone()))
        .expect("Failed to register yellow votes counter!");

    prometheus.registry()
        .register(Box::new(RED_VOTES_COUNTER.clone()))
        .expect("Failed to register red votes counter!");

    rocket::ignite()
        .attach(prometheus.clone())
        .mount("/", routes![index, version, slow, fail, vote])
        .mount("/metrics", prometheus).launch();
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
        for current in 1..250 {
            info!("Current loop index: {}", current);
            let result = create_error_status();
            assert!(result.code >= 500 && result.code <= 511)
        }

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
    fn test_vote_has_valid_color_yellow_returns_ok(){
        let result = vote(Some(String::from("yellow")));

        assert!(result.is_ok());
    }

    #[test]
    fn test_vote_has_invalid_color_returns_err(){
        let result = vote(Some(String::from("yes")));

        assert!(result.is_err());
    }

    #[test]
    fn test_vote_has_empty_string_returns_err(){
        let result = vote(Some(String::from("")));

        assert!(result.is_err());
    }

    #[test]
    fn test_api_call_voted_green_returns_expected_body_and_status(){
        use rocket::local::Client;
        use rocket::http::{ContentType, Status};
        
        let rocket = rocket::ignite().mount("/", routes![vote]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.post("/vote/green").dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        // assert!(response.headers().get_one("X-Special").is_some());
        assert_eq!(response.body_string(), Some("You voted: green".into()));
    }

    #[test]
    fn test_api_call_voted_red_returns_expected_body_and_status(){
        use rocket::local::Client;
        use rocket::http::{ContentType, Status};
        
        let rocket = rocket::ignite().mount("/", routes![vote]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.post("/vote/red").dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        // assert!(response.headers().get_one("X-Special").is_some());
        assert_eq!(response.body_string(), Some("You voted: red".into()));
    }

    #[test]
    fn test_api_call_voted_yellow_returns_expected_body_and_status(){
        use rocket::local::Client;
        use rocket::http::{ContentType, Status};
        
        let rocket = rocket::ignite().mount("/", routes![vote]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.post("/vote/yellow").dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        // assert!(response.headers().get_one("X-Special").is_some());
        assert_eq!(response.body_string(), Some("You voted: yellow".into()));
    }

    #[test]
    fn test_api_call_fail_returns_failed_status(){
        use rocket::local::Client;
        
        let rocket = rocket::ignite().mount("/", routes![fail]);
        let client = Client::new(rocket).expect("valid rocket instance");
        
        for current in 1..250 {
            info!("Current loop index: {}", current);

            let response = client.get("/fail").dispatch();
        
            assert!(response.status().code >= 404 || response.status().code <= 511);
        }
    }

    #[test]
    fn test_version_call_returns_expected_string_and_status(){
        use regex::Regex;

        use rocket::local::Client;
        use rocket::http::{ContentType, Status};
        
        let rocket = rocket::ignite().mount("/", routes![version]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/version").dispatch();
        
        assert_eq!(response.status(), Status::Accepted);
        assert_eq!(response.content_type(), Some(ContentType::Plain));


        let re = Regex::new(r"\d{2}.\d{2}.\d{2}").unwrap();

        let asdf = response.body_string().unwrap_or("What".to_string());
        assert!(asdf.contains("version"));
        assert!(asdf.contains("@"));
        assert!(re.is_match(&asdf));
    }

    #[test]
    fn test_root_call_is_hello_world_response(){
        use rocket::local::Client;
        use rocket::http::{ContentType, Status};
        
        let rocket = rocket::ignite().mount("/", routes![index]);
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        
        assert_eq!(response.status(), Status::Accepted);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        // assert!(response.headers().get_one("X-Special").is_some());
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
