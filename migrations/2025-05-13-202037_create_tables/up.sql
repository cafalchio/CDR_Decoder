-- Your SQL goes here

-- Create cdr_files table
CREATE TABLE cdr_files (
    id SERIAL PRIMARY KEY,
    filename TEXT UNIQUE NOT NULL,
    processed_at TIMESTAMP DEFAULT NOW()
);

-- Create cdr_blocks table
CREATE TABLE cdr_blocks (
    id SERIAL PRIMARY KEY,
    file_id INTEGER NOT NULL REFERENCES cdr_files(id),
    block_type TEXT NOT NULL,
    block_index INTEGER NOT NULL,
    parsed_data JSONB
);