# Using Rust stable release as base image
FROM rust:1.72.0

#switch working directory to 'app' (i.e. 'cd app')
#'app' directry will be created by Docker dne
WORKDIR /app
#Install required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
#Copy all files from working environment to Docker image
COPY . .
#build the binary
#use release profile for SPEEEEEED
RUN cargo build --release
#launch binary when 'docker run' is executed
ENTRYPOINT ["./target/release/armstrong"]