#!/bin/bash

# Format the Rust Code
cargo fmt

# Name of the Docker container
container_name="oracledb_container"

# Build the Docker image
docker build . -t oracledb

# Check if the build was successful
if [ $? -eq 0 ]; then
    echo "Docker image built successfully."
    
    # Check if the container already exists
    if [ $(docker ps -a -q -f name=^/${container_name}$) ]; then
        # Container exists, so just start it
        echo "Starting existing container."
        docker start ${container_name}
    else
        # Container does not exist, run a new container
        echo "Running a new container."
        docker run -p 8080:8080 --env-file .env -v $(pwd):/usr/src/oracledb --name ${container_name} oracledb:latest
    fi
else
    echo "Docker image build failed."
fi