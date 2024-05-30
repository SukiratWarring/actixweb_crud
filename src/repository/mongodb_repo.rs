use std::{env, string};
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::USER;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<USER>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv::dotenv().ok();

        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("chatapp_db");
        let col = db.collection::<USER>("USER");
        return MongoRepo { col };
    }

    pub async fn create_user_handler(&self, new_user: USER) -> Result<InsertOneResult, Error> {
        let new_doc = USER {
            id: None,
            name: new_user.name,
            location: new_user.location,
            message: new_user.message,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
    pub async fn get_user_handler(&self, id: &String) -> Result<USER, String> {
        let converted_id = ObjectId::parse_str(id).expect("Error while converting id to object id");
        let filter = doc! {"_id":converted_id};
        let user: Result<USER, String> = match self.col.find_one(filter, None).await {
            Ok(Some(u)) => Ok(u),
            Ok(None) => Err("User detail not found".to_string()),
            Err(e) => Err(format!("Error getting user's detail: {}", e)),
        };
        return user;
    }
    pub async fn delete_user_handler(&self, id: &String) -> Result<USER, String> {
        let converted_id = ObjectId::parse_str(id).expect("Error while converting id to object id");
        println!("herer {}", converted_id);
        let filter = doc! {"_id":converted_id};
        let res: Result<USER, String> = match self.col.find_one_and_delete(filter, None).await {
            Ok(r) => Ok(r.unwrap()),
            Err(e) => Err(format!("Error getting user's detail: {}", e)),
        };
        return res;
    }
}
