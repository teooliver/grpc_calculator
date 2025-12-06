# Commands:

- Without reflection we need to send the `proto` flag
grpcurl -plaintext -proto ./proto/calculator.proto -d '{"a": 2, "b": 3}' '[::1]:50051' calculator.Calculator.Add

- After setting up reflection we don't need to add the `proto` flag as the server will know
where to look by it self
grpcurl -plaintext -d '{"a": 2, "b": 3}' '[::1]:50051' calculator.Calculator.Add

# Notes
- Check the build.rs to understand how `tonic` uses `::compile_protos` and
`::configure` to setup things automatically

# Tools
- grpcurl
- grpcui
