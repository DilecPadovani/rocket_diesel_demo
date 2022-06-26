# Stage1 : fetch needed dependecies
FROM rustlang/rust:nightly as dep-fetcher
WORKDIR /app
RUN cargo install cargo-chef
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

# Stage2: build needed dependecies
FROM rustlang/rust:nightly as dep-builder
WORKDIR /app
# RUN cargo install cargo-chef
COPY --from=dep-fetcher /usr/local/cargo /usr/local/cargo
COPY --from=dep-fetcher app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage3: compile the app
FROM rustlang/rust:nightly as builder
COPY . /app
WORKDIR /app
COPY --from=dep-builder app/target target
COPY --from=dep-builder /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# Stage4: run the app
# distroless -> this is one of the lightest linux images in the world (from Google) 
FROM debian:stable-slim as runtime

RUN apt-get update -y
RUN apt-get install libpq-dev -y
COPY Rocket.toml Rocket.toml
COPY --from=builder /app/target/release/rocket_diesel_demo /app/rocket_diesel_demo
WORKDIR /app
CMD ["./rocket_diesel_demo"]    