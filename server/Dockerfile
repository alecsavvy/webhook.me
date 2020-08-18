FROM rust:1.31

WORKDIR /usr/src/server
COPY . .

RUN cargo install --path .

CMD ["server"]