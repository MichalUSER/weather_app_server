use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client, Collection};

use crate::Temp;
use crate::load_env::{mongodb_name, mongodb_uri};

#[derive(Clone)]
pub struct Mongo {
    client: Client,
    curr_coll: Collection<Temp>,
}

impl Mongo {
    pub async fn new() -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(mongodb_uri()).await?;
        let client = Client::with_options(client_options)?;
        client
            .database("weather-app")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        let curr_coll = client.database("weather-app").collection::<Temp>(mongodb_name().as_str());
        Ok(Self { client, curr_coll })
    }

    pub async fn add_temp(&self, temp: Temp) -> mongodb::error::Result<()> {
        let curr_coll_ref = self.curr_coll.clone();
        curr_coll_ref.insert_one(temp, None).await?;
        Ok(())
    }

    pub async fn find_temps(&self, day: i32) -> mongodb::error::Result<Vec<Temp>> {
        let filter = doc! { "d": day };
        let cursor = match self.curr_coll.find(filter, None).await {
            Ok(cursor) => cursor,
            Err(_) => return Ok(vec![]),
        };
        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
