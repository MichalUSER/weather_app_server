use crate::{Mongo, SharedTemp, Temp};
use warp::http::StatusCode;

pub async fn add_temp_post(
    temp: Temp,
    m: Mongo,
    last_temp: SharedTemp,
) -> Result<impl warp::Reply, warp::Rejection> {
    *last_temp.lock().unwrap() = temp.clone();
    m.add_temp(temp).await.unwrap();
    Ok(warp::reply::with_status("ok", StatusCode::OK))
}

pub async fn add_last_temp_post(
    temp: Temp,
    m: Mongo,
    last_temp: SharedTemp,
) -> Result<impl warp::Reply, warp::Rejection> {
    *last_temp.lock().unwrap() = temp.clone();
    m.add_last_temp(temp).await.unwrap();
    Ok(warp::reply::with_status("ok", StatusCode::OK))
}

pub async fn temps_get(m: Mongo, month: i32, day: i32) -> Result<impl warp::Reply, warp::Rejection> {
    return match m.find_temps(month, day).await {
        Ok(temps) => Ok(warp::reply::json(&temps)),
        Err(_) => Err(warp::reject()),
    };
}

pub async fn last_temp_get(last_temp: SharedTemp) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&*last_temp))
}

pub async fn last_week_get(m: Mongo) -> Result<impl warp::Reply, warp::Rejection> {
    return match m.last_week().await {
        Ok(temps) => Ok(warp::reply::json(&temps)),
        Err(_) => Err(warp::reject()),
    };
}
