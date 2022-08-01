## lapce-go

The go language server gopls must be available on the path.
Install the go language server by `go install golang.org/x/tools/gopls@latest`.

### how to build

``` bash
rustup target add wasm32-wasi
cargo build
```