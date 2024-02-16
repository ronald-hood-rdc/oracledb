#!/bin/bash

# Format the Rust Code
cargo fmt

# Build the Docker image
docker build . -t oracledb

# Check if the build was successful
if [ $? -eq 0 ]; then
    echo "Docker image built successfully."
    # Run the Docker container using the .env file for environment variables
    docker run --env-file .env -v $(pwd):/usr/src/oracledb oracledb:latest

else
    echo "Docker image build failed."
fi
