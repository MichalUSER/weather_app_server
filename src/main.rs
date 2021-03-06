use std::sync::{Arc, Mutex};
use warp::Filter;

use load_env::{load, url};
use mongo::Mongo;
use routes::*;
use temp::Temp;

mod load_env;
mod mongo;
mod routes;
mod temp;

type SharedTemp = Arc<Mutex<Temp>>;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    load();
    let m = Mongo::new().await?;
    let last_temp = Arc::new(Mutex::new(m.last_temp.clone()));
    let m_filter = warp::any().map(move || m.clone());
    let last_temp_filter = warp::any().map(move || last_temp.clone());

    let add_temp = warp::post()
        .and(warp::path("add_temp"))
        .and(warp::body::json())
        .and(m_filter.clone())
        .and(last_temp_filter.clone())
        .and_then(add_temp_post);

    let add_last_temp = warp::post()
        .and(warp::path("add_last_temp"))
        .and(warp::body::json())
        .and(m_filter.clone())
        .and(last_temp_filter.clone())
        .and_then(add_last_temp_post);

    let temps = warp::get()
        .and(warp::path("temps"))
        .and(m_filter.clone())
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(temps_get);

    let last_week = warp::get()
        .and(warp::path("last_week"))
        .and(m_filter.clone())
        .and_then(last_week_get);

    let last_days = warp::get()
        .and(warp::path("last_days"))
        .and(m_filter.clone())
        .and(warp::path::param())
        .and_then(last_days_get);

    let get_temps = warp::get()
        .and(warp::path("last_temp"))
        .and(last_temp_filter.clone())
        .and_then(last_temp_get);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"]);
    let routes = add_temp
        .or(add_last_temp)
        .or(get_temps)
        .or(temps)
        .or(last_week)
        .or(last_days)
        .with(cors);
    warp::serve(routes).run(url()).await;

    Ok(())
}
