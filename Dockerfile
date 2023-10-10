FROM rust:1.73

WORKDIR /usr/app

COPY . ./

RUN cargo build --release

EXPOSE 8080

CMD ./target/release/wik-dps-tp01