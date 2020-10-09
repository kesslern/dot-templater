#!/bin/bash

VERSION=$1

if [ -z "$VERSION" ]
then
    echo "No release version provided."
    exit 1
fi

echo Creating release for dot-templater v$VERSION

mkdir dot-templater
sed -i -e "s/^version = .*/version = \"$VERSION\"/" Cargo.toml
make
make release
cp target/release/dot-templater dot-templater/
cp LICENSE dot-templater/
cp README.md dot-templater/
tar czf dot-templater-$VERSION-linux-x64.tar.gz dot-templater
rm -rf dot-templater/
