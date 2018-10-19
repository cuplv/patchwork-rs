export RUST_TEST_THREADS=1
cargo build               --verbose
cargo test typing    -j 1 --verbose
export FUNGI_VERBOSE_REDUCE=1
cargo test reduction -j 1 -- --nocapture
#cargo test           -j 1 --verbose
