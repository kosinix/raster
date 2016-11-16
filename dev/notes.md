# Testing

cargo test


# Publishing

run: cargo publish


# Updating documentation

git checkout master
cargo doc --no-deps --open

## Windows
mkdir target\doc\out
xcopy tests\out target\doc\out /e