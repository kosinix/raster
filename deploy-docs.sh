#!/bin/bash

set -o errexit -o nounset

: ${TRAVIS:?"This should only be run on Travis CI."}

rev=$(git rev-parse --short HEAD)

git init
git config user.name "Francesca Frangipane"
git config user.email "francesca@comfysoft.net"

git remote add upstream "https://$GH_TOKEN@github.com/kosinix/raster.git"
git fetch upstream
git reset upstream/gh-pages

rm -rf in out docs
mv tests/in in
mv tests/out out
rm out/.gitignore
mv target/doc docs

git add -A in out docs
git commit -m "Updated docs for ${rev}"
git push --force --quiet upstream HEAD:gh-pages
