#!/usr/bin/sh
# Script for re-generating the code using Openapi-generator
# Includes modifications to fixup known bugs.
sudo rm -rf openapi/out/rust
sudo rm -rf src/

docker run --rm \
  -v ${PWD}/openapi:/local openapitools/openapi-generator-cli generate \
  -i /local/three-hourly_1_0_1.yaml \
  -g rust \
  --generate-alias-as-model \
  -o /local/out/rust

sudo chown -R $USER:$USER openapi/out/*

cp openapi/modifications/modified_configuration.rs openapi/out/rust/src/apis/configuration.rs
cp openapi/modifications/modified_spotdata_three_hourly_api.rs openapi/out/rust/src/apis/spotdata_three_hourly_api_api.rs

cp -rf openapi/out/rust/src/ .
cp -rf openapi/out/rust/docs/ .
cp -f openapi/out/rust/README.md API_README.md

# Modifications to workaround multiple api key bug
