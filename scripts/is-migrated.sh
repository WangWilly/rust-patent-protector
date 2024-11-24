#!/bin/bash

export PROJ_DIR=$(pwd)/backend

isDev=$(echo $1 | tr '[:upper:]' '[:lower:]')

# Load environment variables from the .env file
if [ "$isDev" = "dev" ]; then
    echo "Running in development mode"
    set -a && source ${PROJ_DIR}/dev.env && set +a

    cd ${PROJ_DIR}
    if [ "$(diesel migration pending)" = "true" ]; then
        echo "Database is not migrated"
        exit 1
    else
        echo "Database is migrated"
        exit 0
    fi
else
    echo "Running in production mode"

    if [ "$(diesel migration pending)" = "true" ]; then
        echo "Database is not migrated"
        exit 1
    else
        echo "Database is migrated"
        exit 0
    fi
fi
