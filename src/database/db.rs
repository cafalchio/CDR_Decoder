use crate::database::schema::{cdr_blocks, cdr_files};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use serde;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Queryable, Serialize, Deserialize, Debug, Selectable)]
#[diesel(table_name = cdr_files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CdrFiles {
    pub id: i32,
    pub filename: Option<String>,
    pub processed_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = cdr_files)]
pub struct NewCdrFile<'a> {
    pub filename: Option<&'a str>,
    pub processed_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Selectable)]
#[diesel(table_name = cdr_blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CdrBlocks {
    pub id: i32,
    pub file_id: Option<i32>,
    pub block_type: String,
    pub block_index: i32,
    pub parsed_data: Option<Value>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = cdr_blocks)]
pub struct NewCdrBlock {
    pub file_id: i32,
    pub block_type: String,
    pub block_index: i32,
    pub parsed_data: Option<Value>, // This will be the JSONB field
}
