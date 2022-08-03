# build
echo "Building..."
cargo clean
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
echo "Done building."

# deploy
echo "Deploying..."
if [ ! -d  ~/.lapce/plugins/lapce-go/ ]; then
    mkdir -p  ~/.lapce/plugins/lapce-go/
fi
cp ./target/wasm32-wasi/debug/lapce-go.wasm ~/.lapce/plugins/lapce-go/lapce-go.wasm
cp ./plugin.toml ~/.lapce/plugins/lapce-go/plugin.toml
echo "Done deploying."