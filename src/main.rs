#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use rocket::{
    get, launch, catch, catchers, routes,
    serde::{Deserialize, Serialize},
    serde::json::{json, Value}
};

#[cfg(test)] mod tests;


#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error".to_string(),
        "reason": "Resource not found".to_string(),
    })
}

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        status,
    ])
    .register("/", catchers![not_found])
}
