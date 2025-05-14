use crate::database::db::{CdrFiles, NewCdrBlock, NewCdrFile};
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{insert_into, QueryResult};

use super::db::CdrBlocks;

pub fn insert_cdr_file(conn: &mut PgConnection, filename: &str) -> QueryResult<CdrFiles> {
    use crate::database::schema::cdr_files;

    let new_file = NewCdrFile {
        filename: Some(filename),
        processed_at: Some(Utc::now().naive_utc()),
    };

    insert_into(cdr_files::table)
        .values(&new_file)
        .on_conflict(cdr_files::filename)
        .do_update()
        .set(cdr_files::processed_at.eq(Utc::now().naive_utc()))
        .get_result::<CdrFiles>(conn)
}

pub fn insert_cdr_block(conn: &mut PgConnection, block: NewCdrBlock) -> QueryResult<CdrBlocks> {
    use crate::database::schema::cdr_blocks;

    insert_into(cdr_blocks::table)
        .values(NewCdrBlock {
            file_id: block.file_id,
            block_type: block.block_type.to_string(), // Convert &str to String
            block_index: block.block_index,
            parsed_data: block.parsed_data,
        })
        .get_result(conn)
}
