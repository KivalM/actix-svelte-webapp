### This dockerfile aims to provide a testing reader image for the project
### It is not intended to be used in production as this setup goes against the principles of containerization
### 
### It is merely created to provide a testing environment for the project without manually installing the required dependencies
### and database configurations on the host machine
###

# Use an official Rust image as the base image
FROM rust:latest as rust-builder

# Set the working directory to /app/backend
WORKDIR /app/backend

# Copy the Rust app files to the container
COPY ./backend .

# Compile the Rust app
RUN cargo build --release

# Use an official Node.js image as the base image
FROM node:19.5 as node-builder

# Set the working directory to /app/frontend
WORKDIR /app/frontend

# Copy the frontend app files to the container
COPY ./frontend ./

# Install the Yarn dependencies
RUN yarn install

# Build the Yarn app
RUN yarn build

# Use an official debian image as the base image
FROM debian:bullseye

# Install the required packages
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq5 postgresql postgresql-contrib\
    && rm -rf /var/lib/apt/lists/*

# Set the working directory to /app
WORKDIR /app

# Copy the Rust binary from the rust-builder stage
COPY --from=rust-builder /app/backend/target/release/backend ./backend

# Copy the Yarn build files from the node-builder stage
COPY --from=node-builder /app/frontend/build ./frontend/build

# run the postgresql service
RUN mkdir -p /var/run/postgresql && chown -R postgres:postgres /var/run/postgresql

USER postgres

# remove the default postgresql database
RUN rm -rf /var/lib/postgresql/13/main 

# initialize the postgresql database
RUN /usr/lib/postgresql/13/bin/initdb /var/lib/postgresql/13/main 

# expose the port
EXPOSE 8000

### Main configuration

## Set the environment variables
# enable logging at an info level
# ENV RUST_LOG=info
# enable backtraces
ENV RUST_BACKTRACE=1
# set the timezone to UTC
ENV TZ=Etc/UTC
# a basic postgresql url
ENV DATABASE_URL=postgresql://postgres:postgres@localhost:6543/postgres
# a variable for our rust app to know that it is running in a docker container
ENV DOCKER=true
ENV ENV=prod

# Run the Rust app
# we will do the db configuration at runtime
CMD /usr/lib/postgresql/13/bin/postgres -D /var/lib/postgresql/13/main -p 6543 & \
    ./backend