# wik-dps-tp03 (Rust)

## Start project 

```sh
docker compose up
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