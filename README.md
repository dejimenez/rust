# rust

## gRPC

To implement gRPC I used `tonic` library. To use this library it depends on `tokio` library version `0.2.24`, it did not work with a higher version of `tokio`, it also uses `prost 0.6.1` to generate the code from `.proto` files, it can not be higher version either.

To know the dependencies of the libraries I ran `cargo tree -d`

```
https://blog.logrocket.com/rust-and-grpc-a-complete-guide/
https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
```

## diesel

```
https://diesel.rs/guides/getting-started/
```

If there is an error while running `diesel setup` you can follow this installation

Brew install libpq
Install command: `$ brew install libpq`. Postgres C API library.

Brew makes it easy to install. There’s a small catch though—libpq won’t install itself in the /usr/local/bin directory. To make that happen, you need to run: `brew link --force libpq`. This will symlink all the tools (not just libpq) into the /usr/local/bin directory. You’re now ready to run `psql` and start connecting.

## mongodb

I follow this post

```
https://developer.mongodb.com/quickstart/rust-crud-tutorial
```

It shows how you connect your Rust application to a MongoDB cluster. Then it shows how to do Create, Read, Update, and Delete (CRUD) operations on a collection. Finally, it covers how to use `serde` to map between MongoDB's BSON documents and Rust structs.

We have here the same issue with tokio we had in gRPC example

## tokio

## actix
