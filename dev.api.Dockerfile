FROM rust:nightly
RUN apt install -y libpq-dev
COPY Rocket.toml .
COPY cargo.toml .
COPY diesel.toml .
COPY src .
COPY migrations .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel migration run --database-url=postgres://postgres:password_secure@localhost/test_app_db
RUN cargo build --release
ENTRYPOINT ["target/release/rocket_diesel_demo"]