grpc-client
============

Initial UI using druid.

Outcomes:

* Drop a gRPC spec onto the UI
* Allow user to specify payload / headers / cert
* Allow user to specify service endpoint
* Display response

Dependencies:

- Need to be on nightly rust
- Need rustc-dev component
```
rustup default nightly
rustup component add rust-src rustc-dev llvm-tools-preview
```