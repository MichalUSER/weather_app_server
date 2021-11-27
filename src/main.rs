use std::sync::{Arc, Mutex};
use warp::Filter;

use crate::routes::{add_temp_post, last_temp_get, temps_get};
use mongo::Mongo;
use temp::Temp;

mod mongo;
mod routes;
mod temp;

type SharedTemp = Arc<Mutex<Temp>>;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let m = Mongo::new().await?;
    let m_filter = warp::any().map(move || m.clone());
    let last_temp = Arc::new(Mutex::new(Temp::default()));
    let last_temp_filter = warp::any().map(move || last_temp.clone());

    let add_temp = warp::post()
        .and(warp::path("add_temp"))
        .and(warp::body::json())
        .and(m_filter.clone())
        .and(last_temp_filter.clone())
        .and_then(add_temp_post);

    let temps = warp::get()
        .and(warp::path("temps"))
        .and(m_filter.clone())
        .and(warp::path::param())
        .and_then(temps_get);

    let get_temps = warp::get()
        .and(warp::path("last_temp"))
        .and(last_temp_filter.clone())
        .and_then(last_temp_get);

    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_methods(vec!["GET", "POST"]);
    let routes = add_temp.or(get_temps).or(temps).with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
