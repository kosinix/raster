# Testing

cargo test

## Target Specific Test

cargo test --test integration_tests


# Updating documentation

git checkout master
cargo doc --no-deps --open

## Windows Copy Images
mkdir target\doc\out
xcopy tests\out target\doc\out /e

## Windows Copy Doc to gh-pages
rmdir "C:\Users\Lenovo G410\Desktop\rust\tmp" /S /Q
mkdir "C:\Users\Lenovo G410\Desktop\rust\tmp"
xcopy target\doc "C:\Users\Lenovo G410\Desktop\rust\tmp" /e

git checkout gh-pages

xcopy "C:\Users\Lenovo G410\Desktop\rust\tmp" "%cd%" /e

git checkout master

# Publishing

run: cargo publish