# CDR_Decoder




#### Development

## Requirements

 - Rust
 - uv
 - node 22.15

## Install

* RUST
    - cargo build
    - cargo run (fast compilation) or cargo run --release (better app performance)

* Maturin (PyO3)
    - uv venv
    - source .venv/bin/activate
    - uv sync
    - maturin develop (will create the python package)
    - python python/main.py 

* Cargo utilities
    - cargo fmt - (code format)
    - cargo check - (check code )


* To check instalation:
- from base folder "cargo test"
- from frontend folder "npm run dev"


 ### Project Structure

- src/datatypes 
    Contain all implementations of types described in the documentation
- src/datatypes/primities - implement primitive types
- src/datatypes/charging_fields - contains all datatypes in the documentation as structs
- src/datatypes/charging_fields_impl - implementation of the charging fields


- src/datablocks 
    Besides the mod and blocks it implements all CDR types found in the documentation
    - blocks - implement the block type that will match the blocks

- core - process_file - helper file with functions to process files

- lib.rs - this file is used to bind rust to python using Pyo3 or tauri

- main.rs - File that is called when run cargo run or cargo run --release


if compilation fails: sudo apt-get install libpq-dev

    


    