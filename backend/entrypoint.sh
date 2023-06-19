#!/bin/bash

server="${SERVER:-backend_api}"
prevent_auto_migrate="${PREVENT_AUTO_MIGRATE:-false}"

POSTGRES_PASSWORD=$(</run/secrets/POSTGRES_PASSWORD)
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"
export ROCKET_DATABASES="{db={url=\"${DATABASE_URL}\"}}"

if [[ "$server" == "filewatcher" ]]
then
    echo $ROCKET_DATABASES
    echo $DATABASE_URL

    rm -rf ./static

    # Start the filewatcher at appropriate path
    ./target/release/filewatcher /home/.watchdir

else
    echo $ROCKET_DATABASES
    echo $DATABASE_URL

    # Start the server
    ./target/release/backend_api

fi
