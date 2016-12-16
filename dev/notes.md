# Testing

cargo test

## Target Specific Test

cargo test --test integration_tests


# Updating documentation

git checkout master
cargo doc --no-deps --open

## Copy Test Images to a Temp Dir - Windows 
mkdir "D:\tmp\in"
mkdir "D:\tmp\out"
xcopy tests\in "D:\tmp\in" /e
xcopy tests\out "D:\tmp\out" /e

## Copy Test Images to gh-pages - Windows

git checkout gh-pages

rmdir "%cd%\in" /S /Q
rmdir "%cd%\out" /S /Q

xcopy "D:\tmp\in" "%cd%\in" /e
xcopy "D:\tmp\out" "%cd%\out" /e

# Publishing

run: cargo publish