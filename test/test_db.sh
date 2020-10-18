#!/bin/bash
rm -f $DATABASE_URL/assets/assets.db

cd db/cache/
diesel database setup --database-url $DATABASE_URL/assets/assets.db --config-file ./diesel.toml
diesel migration run --database-url $DATABASE_URL/assets/assets.db --config-file ./diesel.toml

cd ../..

cargo test create_tables

cargo test test_asset_cache