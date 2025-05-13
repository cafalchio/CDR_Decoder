// @generated automatically by Diesel CLI.

diesel::table! {
    cdr_blocks (id) {
        id -> Int4,
        file_id -> Nullable<Int4>,
        block_type -> Text,
        block_index -> Int4,
        parsed_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    cdr_files (id) {
        id -> Int4,
        filename -> Nullable<Text>,
        processed_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(cdr_blocks -> cdr_files (file_id));

diesel::allow_tables_to_appear_in_same_query!(
    cdr_blocks,
    cdr_files,
);
