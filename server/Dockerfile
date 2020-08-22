FROM rust:1.45.2

WORKDIR /usr/src/server
COPY . .

RUN cargo install --path .

CMD ["server"]