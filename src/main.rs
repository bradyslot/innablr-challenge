#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use rocket::{
    get, launch, catch, catchers, routes,
    serde::json::{json, Value}
};

// set attribute to run tests module
#[cfg(test)] mod tests;

// catch requests to unknown routes
#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error".to_string(),
        "reason": "Resource not found".to_string(),
    })
}

// return hello world as a string from the root route
#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

// return a json object with the name of the app and version details
#[get("/status")]
fn status() -> Value {
    json!({
        env!("CARGO_PKG_NAME").to_string(): [
            {
                "version": env!("CARGO_PKG_VERSION").to_string(),
                "description": env!("CARGO_PKG_DESCRIPTION").to_string(),
                "sha": env!("VERGEN_GIT_SHA").to_string()
            }
        ]
    })
}

// launch the rocket instance which serves as the main entry point
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        status,
    ])
    .register("/", catchers![not_found])
}
