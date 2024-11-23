#!/bin/bash

export PROJ_DIR=$(pwd)/backend

isDev=$(echo $1 | tr '[:upper:]' '[:lower:]')

# Load environment variables from the .env file
if [ "$isDev" = "dev" ]; then
    echo "Running in development mode"
    set -a && source ${PROJ_DIR}/dev.env && set +a

    cd ${PROJ_DIR}
    diesel migration run
else
    echo "Running in production mode"

    diesel migration run
    # sleep infinity
fi
