#Build stage
# Using latest Rust stable release as base image and
# Luca palmieri's cargo-chef as build stager for diff efficiency (caching)
FROM lukemathwalker/cargo-chef:latest-rust-1.91.1 as chef

#switch working directory to 'app' (i.e. 'cd app')
#'app' directry will be created by Docker dne
WORKDIR /app
#Install required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

From chef as planner
#Copy all files from working environment to Docker image
COPY . .

# Compute a pseudo lock file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build the project dependencies, not the application
RUN cargo chef cook --release --recipe-path recipe.json
#up to here, if the dependency tree stays the same,
# all layers should be cached
COPY . .
#access offline sqlx data
ENV SQLX_OFFLINE=true
#build the binary
#use release profile for SPEEEEEED
RUN cargo build --release --bin armstrong

#Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app

#install OpenSSL - since it's dynamically linked by some of our dependencies
# install ca-certificates - needed to verify TLS certificates when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    #Clean Up Crap
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

#Copy the compiled binary from the build environment to the runtime environment
COPY --from=builder /app/target/release/armstrong armstrong
#runtime needs configuration file
COPY configuration configuration
#set environment for proper configuration file
ENV APP_ENVIRONMENT=production
#launch binary when 'docker run' is executed
ENTRYPOINT ["./armstrong"]