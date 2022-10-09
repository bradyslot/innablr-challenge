#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use reqwest;
use rocket::{
    catch, catchers,
    fairing::{Fairing, Info, Kind},
    get,
    http::{Cookie, CookieJar},
    launch, post,
    response::status::Created,
    routes, uri, Data, Request, State,
};
use rocket::serde::{Deserialize, Serialize};
use vergen::{vergen, Config, ShaKind};

#[cfg(test)] mod tests;


fn get_application_version() -> String {
    return env!("CARGO_PKG_VERSION").to_string();
}

fn get_application_description() -> String {
    return env!("CARGO_PKG_DESCRIPTION").to_string();
}

fn get_current_git_commit() -> String {
    return env!("VERGEN_SHA").to_string();
}

#[derive(Serialize, Debug, Clone)]
struct MyApplicaion {
    version: get_application_version,
    description: String,
    sha: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/status")]
fn status() -> String {
    "All good.".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        status,
    ])
}
