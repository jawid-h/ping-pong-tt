FROM rust:latest as build

# Create a new directory in our docker container and set it as the working directory.
WORKDIR /usr/src/app

# Install necessary dependencies
RUN apt-get update \
    && apt-get install -y cmake clang libclang-dev

# Copy over our Cargo.toml and Cargo.lock file from each workspace.
COPY ./Cargo.toml ./Cargo.lock ./

# Copy over the source code from each workspace.
COPY ./client ./client
COPY ./server ./server
COPY ./common ./common
COPY ./cli ./cli

# Build our application. This will build the cli.
RUN cargo build --release --bin cli

# Start a new build stage so that we can minimise our layer size.
FROM debian:buster-slim

WORKDIR /usr/src/app

# Copy over the build artifact from the previous step and set our command to run the binary.
COPY --from=build /usr/src/app/target/release/cli /usr/local/bin/cli

ENTRYPOINT ["cli"]
