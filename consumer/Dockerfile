FROM rust:1.62.1

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["consumer"]
