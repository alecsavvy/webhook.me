FROM rust:1.45.2

RUN rm -rf /usr/src/server
WORKDIR /usr/src/server
COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["server"]