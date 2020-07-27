FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features mysql

RUN cargo install cargo-watch

ENV APP=/app

WORKDIR $APP

COPY . $APP

EXPOSE 8000
