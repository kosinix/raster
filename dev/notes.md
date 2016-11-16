# Testing

cargo test

## Target Specific Test

cargo test --test integration_tests


# Updating documentation

git checkout master
cargo doc --no-deps --open

## Windows
mkdir target\doc\out
xcopy tests\out target\doc\out /e
rmdir "C:\Users\Lenovo G410\Desktop\rust\tmp"
mkdir "C:\Users\Lenovo G410\Desktop\rust\tmp"
xcopy target\doc "C:\Users\Lenovo G410\Desktop\rust\tmp" /e

git checkout gh-pages
git checkout

# Publishing

run: cargo publish