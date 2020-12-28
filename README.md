# rust

## gRPC

To implement gRPC I used `tonic` library. To use this library it depends on `tokio` library version `0.2.24`, it did not work with a higher version of `tokio`, it also uses `prost 0.6.1` to generate the code from `.proto` files, it can not be higher version either.

```
https://blog.logrocket.com/rust-and-grpc-a-complete-guide/
https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
```
