extern crate dotenv;

use std::{env, net::SocketAddr};

pub fn load() {
    dotenv::dotenv().ok();
}

pub fn mongodb_uri() -> String {
    env::var("MONGODB_URI").expect("MONGODB_URI must be set")
}

pub fn mongodb_name() -> String {
    env::var("MONGODB_NAME").expect("MONGODB_NAME must be set")
}

pub fn url() -> SocketAddr {
    SocketAddr::new(env::var("url").expect("url must be set").parse().unwrap(), 8080)
}
