use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client, Collection};

use crate::Temp;
use crate::load_env::{mongodb_name, mongodb_uri};

#[derive(Clone)]
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

    pub async fn find_temps(&self, day: i32) -> mongodb::error::Result<Vec<Temp>> {
        let filter = doc! { "d": day };
        let cursor = match self.curr_coll.clone().find(filter, None).await {
            Ok(cursor) => cursor,
            Err(_) => return Ok(vec![]),
        };
        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
