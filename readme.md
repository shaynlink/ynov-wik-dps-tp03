# wik-dps-tp02 (Rust)

## Start project with docker
```sh
docker build -t wikdpstp02 .

docker run -p -it 8080:8080 wikdpstp02
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