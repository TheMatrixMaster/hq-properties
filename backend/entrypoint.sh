#!/bin/bash

server="${SERVER:-api}"
prevent_auto_migrate="${PREVENT_AUTO_MIGRATE:-false}"

POSTGRES_PASSWORD=$(</run/secrets/POSTGRES_PASSWORD)
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"
export ROCKET_DATABASES="{db={url=\"${DATABASE_URL}\"}}"

if [[ "$server" == "fw" ]]
then
    # Start the filewatcher at appropriate path
    ./target/release/fw /home/.watchdir
    
elif [[ "$server" == "seed_db" ]]
then
    # Start the seeder script
    ./target/release/seed_db

else
    # Start the api server
    ./target/release/api

fi
