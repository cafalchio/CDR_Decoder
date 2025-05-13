use diesel::prelude::*;
use diesel::Queryable;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}




#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CdrFile {
    pub id: i32,
    pub filename: Option<String>,
    pub processed_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct CdrBlock {
    pub id: i32,
    pub file_id: Option<i32>,
    pub block_type: String,
    pub block_index: i32,
    pub parsed_data: Option<serde_json::Value>, 
}