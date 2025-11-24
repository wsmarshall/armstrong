#Build stage
# Using latest Rust stable release as base image
FROM rust:1.91.1 AS builder

#switch working directory to 'app' (i.e. 'cd app')
#'app' directry will be created by Docker dne
WORKDIR /app
#Install required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
#Copy all files from working environment to Docker image
COPY . .
#access offline sqlx data
ENV SQLX_OFFLINE=true
#build the binary
#use release profile for SPEEEEEED
RUN cargo build --release

#Runtime stage
FROM rust:1.91.1 AS runtime

WORKDIR /app
#Copy the compiled binary from the build environment to the runtime environment
COPY --from=builder /app/target/release/armstrong armstrong
#runtime needs configuration file
COPY configuration configuration
#set environment for proper configuration file
ENV APP_ENVIRONMENT=production
#launch binary when 'docker run' is executed
ENTRYPOINT ["./armstrong"]