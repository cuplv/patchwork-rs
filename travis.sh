export RUST_TEST_THREADS=1
cargo build --verbose
cargo test -j 1 --verbose
