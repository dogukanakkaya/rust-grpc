# rust-grpc

```
docker build -t rust-grpc-server .
docker run -dit --name rust-grpc-server -v $(pwd):/usr/src/app -v /usr/src/app/target -p 8080:50051 rust-grpc-server
```

> [BloomRPC](https://github.com/bloomrpc/bloomrpc), GUI client for RPC services.