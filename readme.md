# wik-dps-tp001 (Rust)

## Start project
### Start project without setting port
```sh
cargo run
```

Default port is `8080`

### Start project with setting port
#### Windows
```sh
cargo build

export PING_LISTEN_PORT=3000 && ./target/debug/wok-dps-tp001
```

#### Linux / MacOS
```sh
cargo build

PING_LISTEN_PORT=3000 ./target/debug/wok-dps-tp001
```

# Usage

## Ping
### Request
http://localhost:8080/ping

###  `GET` /ping
```sh
curl http://localhost:8080/ping
```
### Response
```json
200 OK

{
    "Host":"localhost:8080",
    "User-Agent":"curl/8.1.2",
    "Accept":"*/*"
}
```

## 404

### Response
```json
404 not found
```