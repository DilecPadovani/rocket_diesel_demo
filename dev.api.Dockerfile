FROM rustlang/rust:nightly
RUN apt install -y libpq-dev git
WORKDIR /~
COPY Rocket.toml .
COPY Cargo.toml .
COPY diesel.toml .
COPY src src
COPY migrations migrations
RUN cargo build --release
ENTRYPOINT ["target/release/rocket_diesel_demo"]    