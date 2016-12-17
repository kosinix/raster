# Testing

cargo test

## Target Specific Test

cargo test --test integration_tests

# Updating documentation

git checkout master
cargo doc --no-deps --open

# Publishing

run: cargo publish