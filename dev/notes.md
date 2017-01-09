# Testing

cargo test

## Target Specific Test

cargo test --test filter_tests

## Test with output 

cargo test -- --nocapture

# Updating documentation

git checkout master
cargo doc --no-deps --open

# Publishing

run: cargo publish