use chrono::Duration;
use chrono::prelude::*;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client, Collection};

use crate::Temp;
use crate::load_env::{mongodb_name, mongodb_uri};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Mongo {
    client: Client,
    curr_coll: Collection<Temp>,
    last_temp_coll: Collection<Temp>,
    pub last_temp: Temp,
}

async fn get_last_temp(coll: Collection<Temp>) -> mongodb::error::Result<Temp> {
    let cursor = match coll.find_one(doc! {}, None).await {
        Ok(cursor) => cursor,
        Err(e) => return Err(e),
    };
    Ok(cursor.unwrap())
}

impl Mongo {
    pub async fn new() -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(mongodb_uri()).await?;
        let client = Client::with_options(client_options)?;
        client
            .database("weather-app")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        let db = client.database("weather-app");
        let curr_coll = db.collection::<Temp>(mongodb_name().as_str());
        let last_temp_coll = db.collection::<Temp>("last_temp");
        let last_temp = get_last_temp(last_temp_coll.clone()).await.unwrap();
        Ok(Self { client, curr_coll, last_temp_coll, last_temp })
    }

    pub async fn add_temp(&self, temp: Temp) -> mongodb::error::Result<()> {
        self.curr_coll.clone().insert_one(temp.clone(), None).await?;
        self.last_temp_coll.clone().replace_one(doc! {}, temp.clone(), None).await?;

        Ok(())
    }

    pub async fn add_last_temp(&self, temp: Temp) -> mongodb::error::Result<()> {
        self.last_temp_coll.clone().replace_one(doc! {}, temp.clone(), None).await?;

        Ok(())
    }

    pub async fn last_week(&self) -> mongodb::error::Result<Vec<Temp>> {
        let now = Utc::now();
        let week_ago = now - Duration::days(7);
        let filter = doc! { "d": { "$gt": week_ago.day() - 1, "$lt": now.day() + 1 }, "m": { "$in": [ week_ago.month(), now.month() ] } };
        let cursor = match self.curr_coll.clone().find(filter, None).await {
            Ok(cursor) => cursor,
            Err(e) => return Err(e),
        };
        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    pub async fn last_days(&self, days: i64) -> mongodb::error::Result<Vec<Temp>> {
        let now = Utc::now();
        let week_ago = now - Duration::days(days);
        let filter = doc! { "d": { "$gt": week_ago.day() - 1, "$lt": now.day() + 1 }, "m": { "$in": [ week_ago.month(), now.month() ] } };
        let cursor = match self.curr_coll.clone().find(filter, None).await {
            Ok(cursor) => cursor,
            Err(e) => return Err(e),
        };
        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }

    pub async fn find_temps(&self, month: i32, day: i32) -> mongodb::error::Result<Vec<Temp>> {
        let filter = doc! { "d": day, "m": month };
        let cursor = match self.curr_coll.clone().find(filter, None).await {
            Ok(cursor) => cursor,
            Err(_) => return Ok(vec![]),
        };
        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
