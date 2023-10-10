FROM rust:1.73

RUN useradd -ms /bin/bash wikdps

USER wikdps

WORKDIR /usr/app

COPY . ./

RUN cargo build --release

EXPOSE 8080

RUN whoami

CMD ./target/release/wik-dps-tp01