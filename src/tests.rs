#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use rocket::{
    local::blocking::Client,
    http::{Status, Header},
    serde::json::{json, Value}
};

#[test]
fn test_index() {
    let client = Client::tracked(super::rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello World".into()));
}

#[test]
fn test_status() {
    let client = Client::tracked(super::rocket()).expect("valid rocket instance");
    let response = client.get("/status").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json(), Some(json!({
        env!("CARGO_PKG_NAME").to_string(): [
            {
                "version": env!("CARGO_PKG_VERSION").to_string(),
                "description": env!("CARGO_PKG_DESCRIPTION").to_string(),
                "sha": env!("VERGEN_GIT_SHA").to_string()
            }
        ]
    })));
}
