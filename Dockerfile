FROM rust:1.43-slim-buster

WORKDIR /heroku

ADD Cargo.* /heroku/
ADD src/ /heroku/src/

RUN \
    apt update -y && \
    apt install -y pkg-config libssl-dev && \
    cargo build --release

CMD ["cargo", "run", "--release"]
