use crate::database::db::{CdrFiles, NewCdrBlock, NewCdrFile};
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::{insert_into, QueryResult};

const BATCH_SIZE: usize = 1024;

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

pub fn insert_cdr_blocks(conn: &mut PgConnection, blocks: Vec<NewCdrBlock>) {
    use crate::database::schema::cdr_blocks;

    // Split the blocks into chunks of size `batch_size`
    let chunked_blocks = blocks.chunks(BATCH_SIZE);

    // Insert each chunk separately
    for (i, chunk) in chunked_blocks.enumerate() {
        println!("saving {} blocks", BATCH_SIZE * i);
        // Insert the chunk of blocks into the database
        insert_into(cdr_blocks::table)
            .values(chunk)
            .execute(conn)
            .unwrap(); // You can handle errors more gracefully in production
    }
}
