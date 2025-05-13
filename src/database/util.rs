
use diesel::{insert_into, QueryResult};
use chrono::Utc;
use diesel::pg::PgConnection;
use crate::database::db::{NewCdrBlock, NewCdrFile, CdrFiles};
use diesel::RunQueryDsl;

pub fn insert_cdr_file(conn: &mut PgConnection, filename: &str) -> QueryResult<CdrFiles> {
    use crate::database::schema::cdr_files;

    let new_file = NewCdrFile {
        filename: Some(filename),
        processed_at: Some(Utc::now().naive_utc()),
    };

    insert_into(cdr_files::table)
        .values(&new_file)
        .get_result::<CdrFiles>(conn)
}

pub fn insert_cdr_block(conn: &mut PgConnection, block: NewCdrBlock) -> QueryResult<usize> {
    use crate::database::schema::cdr_blocks;

    insert_into(cdr_blocks::table)
        .values(&block)
        .execute(conn)
}