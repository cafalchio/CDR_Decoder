# CDR_Decoder




#### Development

## Requirements

 - Rust
 - uv
 - node 22.15

## Install

* RUST
    - cargo install
    - cargo build
    - cargo run (fast compilation) or cargo run --release (better app performance)

* Maturin (PyO3)
    - uv venv
    - source .venv/bin/activate
    - uv sync
    - maturin develop (will create the python package)
    - python python/main.py 

* Tauri
    - cd /frontend
    - npm install

* To check instalation:
- from base folder "cargo test"
- from frontend folder "npm run dev"
