#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use rocket::{local::blocking::Client, http::{Status, Header,},};
// use base64;

#[test]
fn test_index() {
    let client = Client::tracked(super::rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}

#[test]
fn test_status() {
    let client = Client::tracked(super::rocket()).expect("valid rocket instance");
    let response = client.get("/status").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("All good.".into()));
}
