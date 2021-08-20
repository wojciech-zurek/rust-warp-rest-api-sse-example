# Rust + Warp + Rest Api + Server Sent Events example

## Endpoints

#### GET /api/v1/users

```fish
http get localhost:8080/api/v1/users
HTTP/1.1 200 OK
content-length: 72
content-type: application/json
date: Fri, 20 Aug 2021 15:47:30 GMT

[
    {
        "age": 30,
        "id": "e65c26a6-4db6-4c50-a4ef-6ca5176f179d",
        "name": "Wojtek"
    }
]
```

#### GET /api/v1/users/{userId}

```fish
http get localhost:8080/api/v1/users/e65c26a6-4db6-4c50-a4ef-6ca5176f179d
HTTP/1.1 200 OK
content-length: 70
content-type: application/json
date: Fri, 20 Aug 2021 15:48:44 GMT

{
    "age": 30,
    "id": "e65c26a6-4db6-4c50-a4ef-6ca5176f179d",
    "name": "Wojtek"
}
```

#### POST /api/v1/users
```fish
http post localhost:8080/api/v1/users name=Ola age:=30
HTTP/1.1 201 Created
content-length: 67
content-type: application/json
date: Fri, 20 Aug 2021 15:49:19 GMT

{
    "age": 30,
    "id": "9a5464e8-f593-43eb-b8d3-f079ae080e8f",
    "name": "Ola"
}
```

#### PUT /api/v1/users/{userId}
```fish
http put localhost:8080/api/v1/users/9a5464e8-f593-43eb-b8d3-f079ae080e8f name=Ola age:=29
HTTP/1.1 200 OK
content-length: 67
content-type: application/json
date: Fri, 20 Aug 2021 15:50:41 GMT

{
    "age": 29,
    "id": "9a5464e8-f593-43eb-b8d3-f079ae080e8f",
    "name": "Ola"
}
```

#### DELETE /api/v1/users/{userId}
```fish
http delete localhost:8080/api/v1/users/9a5464e8-f593-43eb-b8d3-f079ae080e8f
HTTP/1.1 200 OK
content-length: 0
date: Fri, 20 Aug 2021 15:51:48 GMT

```

#### GET /api/v1/sse
```fish
http -S localhost:8080/api/v1/sse
HTTP/1.1 200 OK
cache-control: no-cache
content-type: text/event-stream
date: Fri, 20 Aug 2021 15:52:46 GMT
transfer-encoding: chunked

event:system
data:"Hello"

event:user
data:"UserCreated(112a527f-7a97-4d9f-aa34-4b16ef25cd26)"

event:user
data:"UserCreated(0a9c5136-4d6a-4c35-bade-141d8f020688)"

event:user
data:"UserCreated(bccfa787-7c99-4106-8fa4-9845f80fdc19)"

event:user
data:"UserDeleted(bccfa787-7c99-4106-8fa4-9845f80fdc19)"

event:user
data:"UserUpdated(0a9c5136-4d6a-4c35-bade-141d8f020688)"

```

## Download
```fish
    git https://github.com/wojciech-zurek/rust-warp-rest-api-sse-example.git
```

## Run with cargo
```fish
    cd rust-warp-rest-api-sse-example/
    cargo run
    
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/rust-warp-rest-api-sse-example`
 INFO  warp::server > Server::run; addr=127.0.0.1:8080
 INFO  warp::server > listening on http://127.0.0.1:8080 
 INFO  any          > 127.0.0.1:45190 "GET /api/v1/users/e65c26a6-4db6-4c50-a4ef-6ca5176f179d HTTP/1.1" 404 "-" "HTTPie/2.4.0" 114.515µs
 INFO  any          > 127.0.0.1:45192 "GET /api/v1/users HTTP/1.1" 200 "-" "HTTPie/2.4.0" 94.262µs
    
```

