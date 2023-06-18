#!/bin/bash

server="${SERVER:-FLASK}"
prevent_auto_migrate="${PREVENT_AUTO_MIGRATE:-false}"

if [[ "$server" == "FILEWATCHER" ]]
then
    # Start the filewatcher at appropriate path
    $(./target/release/filewatcher)

else
    # Run migrations on the database
    if [ "$prevent_auto_migrate" != true ]
    then
        echo ""
        echo "Auto-running migrations:"
        diesel migration run
    fi

    # Start the server
    $(./target/release/backend_api)
fi