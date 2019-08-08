#! /usr/bin/env bash
set -e

VERSION="v"$(cat Cargo.toml | grep version | awk -F= '{ print $2 }' | sed 's/[",]//g' | tr -d '[[:space:]]')

echo "Tagging version as $VERSION"

git tag $VERSION
git push --tags
