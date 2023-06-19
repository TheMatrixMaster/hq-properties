#!/bin/sh

server="${SERVER:-backend_api}"
prevent_auto_migrate="${PREVENT_AUTO_MIGRATE:-false}"

POSTGRES_PASSWORD=$(<secrets/POSTGRES_PASSWORD)
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}"
export ROCKET_DATABASES="{db={url=\"${DATABASE_URL}\"}}"

if [[ "$server" == "filewatcher" ]]
then
    # Start the filewatcher at appropriate path
    ./target/release/filewatcher /home/.watchdir

elif [[ "$server" == "backend_api" ]]
then
    # Start the server
    ./target/release/backend_api

elif [ "$prevent_auto_migrate" != true ]
then
    # Run migrations on the database
    echo ""
    echo "Auto-running migrations:"
    diesel database setup
    diesel migration run
fi
