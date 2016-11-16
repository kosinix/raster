# Publishing

run: cargo publish

# Updating documentation

git checkout master
cargo doc --no-deps --open

## Windows
xcopy tests\image\out target\doc\out /e