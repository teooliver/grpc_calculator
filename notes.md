# Running

This project contains two binaries, a server and a client.

## Server

```
cargo run --bin server
```

## Client

```
cargo run --bin client
```

# Commands:

- Without reflection we need to send the `proto` flag
```
grpcurl -plaintext -proto ./proto/calculator.proto -d '{"a": 2, "b": 3}' '[::1]:50051' calculator.Calculator.Add
```
- After setting up reflection we don't need to add the `proto` flag as the server will know
where to look by it self
```
grpcurl -plaintext -d '{"a": 2, "b": 3}' '[::1]:50051' calculator.Calculator.Add
```

- grpcui
```
grpcui -plaintext '[::1]:50051'
```

- Unauthenticated
```
grpcurl -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
```

- Authenticated
```
grpcurl -H "Authorization: Bearer some-secret-token"  -emit-defaults -plaintext '[::1]:50051' calculator.Admin.GetRequestCount
```

# Notes
- Check the build.rs to understand how `tonic` uses `::compile_protos` and
`::configure` to setup things automatically

# Tools
- grpcurl
- grpcui


Refs:
https://www.youtube.com/watch?v=kerKXChDmsE
