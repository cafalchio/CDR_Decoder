-- Your SQL goes here

-- Create cdr_files table
CREATE TABLE cdr_files (
    id SERIAL PRIMARY KEY,
    filename TEXT,
    processed_at TIMESTAMP DEFAULT NOW()
);

-- Create cdr_blocks table
CREATE TABLE cdr_blocks (
    id SERIAL PRIMARY KEY,
    file_id INTEGER REFERENCES cdr_files(id),
    block_type TEXT NOT NULL,
    block_index INTEGER NOT NULL,
    parsed_data JSONB
);