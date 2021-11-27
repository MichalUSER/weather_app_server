extern crate dotenv;

use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client, Collection};
use std::env;

use crate::Temp;

pub struct Mongo {
    coll: Collection<Temp>,
}

impl Clone for Mongo {
    fn clone(&self) -> Self {
        Self {
            coll: self.coll.clone(),
        }
    }
}

fn mongodb_uri() -> String {
    env::var("MONGODB_URI").expect("MONGODB_URI must be set")
}

impl Mongo {
    pub async fn new() -> mongodb::error::Result<Self> {
        dotenv::dotenv().ok();
        let client_options = ClientOptions::parse(mongodb_uri()).await?;
        let client = Client::with_options(client_options)?;
        client
            .database("weather-app")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        let coll = client.database("weather-app").collection::<Temp>("temps");
        Ok(Self { coll })
    }

    pub async fn add_temp(&self, temp: Temp) -> mongodb::error::Result<()> {
        let coll_ref = self.coll.clone();
        coll_ref.insert_one(temp, None).await?;
        Ok(())
    }

    pub async fn find_temps(&self, day: i32) -> mongodb::error::Result<Vec<Temp>> {
        let filter = doc! { "d": day };
        let cursor = match self.coll.find(filter, None).await {
            Ok(cursor) => cursor,
            Err(_) => return Ok(vec![]),
        };
        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
