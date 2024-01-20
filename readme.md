# Simple in-memory Key-Value Store 

This is a Key-Value store server written in Rust using the Axum framework.
I was looking for a minimal example of a kv web-server written in Rust, and I couldn't find one that was simple enough. So I decided to write one myself.

## Features

- Key-value storage: The server provides endpoints to get, set, and delete key-value pairs.
- Concurrency: The key-value store is implemented using a `RwLock` to allow concurrent read and write operations.

## Getting Started

1. Clone the repository:

    ```shell
    git clone https://github.com/ryseek/axum-simple-key-value-store.git
    ```

2. Build and run the server:

    ```shell
    cd axum-simple-key-value-store
    cargo run
    ```

3. The server will be running at `http://localhost:3000`.

## API Endpoints

- `GET /get/:key` Retrieves the value associated with the specified key.
- `GET /set/:key/:value` Sets the value for the specified key.
- `GET /del/:key` Deletes the key-value pair associated with the specified key.

## Examples

Once server is running, you can use `curl` to test the API endpoints:

```shell
➜ curl localhost:3000/get/1
Key not found   
➜ curl localhost:3000/set/1/test
OK                                                                          
➜ curl localhost:3000/get/1     
test                                                                        
➜ curl localhost:3000/del/1 
OK       
➜ curl localhost:3000/get/1
Key not found               
```

