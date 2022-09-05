# Flutter<>Rust Proto demo

A proof of concept of calling rust code that uses a generated rust grpc client to connect to a rust server.

Why not use flutter grpc client code instead? Because I want to eventually do advanced things like abstract out the GRPC code so that it may call an "embedded server" instead of always calling out to a remove GPRC server. Then on the flutter side, it doesn't have to change much to have an "embedded server" or keep calling the abstraction to calll the remote grpc server.

Next steps:
  - [ ] See if tor works in any capacity
  - [ ] Embed the service directly into the rust client library
    - [ ] Have the ability to switch between embedded mode and remote grpc mode easily in flutter.

## Server

Run the rust server code:

```
cargo run --bin server
```

## CLI client

While the server is running, test to make sure the CLI can hit it over GRPC

```
cargo run --bin cli
```

## Flutter client

If you change any of the public `src/lib.rs` APIs, generate the dart code again:

First install `flutter_rust_bridge_codegen`  and anything else that might be required [here](http://cjycode.com/flutter_rust_bridge/integrate/deps.html).


At minimum:
```
cargo install flutter_rust_bridge_codegen
```

Then generate:
```
cd mobile && flutter_rust_bridge_codegen --rust-input native/src/api.rs --dart-output lib/bridge_generated.dart
```

Now run the flutter client:

```
flutter run
```
