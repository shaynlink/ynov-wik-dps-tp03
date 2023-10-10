FROM rust:1.73

RUN useradd -ms /bin/bash wikdps

WORKDIR /home/wikdps/app

COPY . ./

RUN cargo build --release

USER wikdps

EXPOSE 8080

CMD ./target/release/wik-dps-tp01