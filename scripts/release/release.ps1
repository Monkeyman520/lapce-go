# preset variables
$origin = $( Get-Location )
$target = "~"
# build
Write-Output "Building..."
cargo clean
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
Write-Output "Done building."

# deploy
Write-Output "Deploying..."
Set-Location $target
$target = $( Get-Location )
$plugin_path="{0}\.lapce\plugins\lapce-go\" -f $target
if (!([System.IO.Directory]::Exists($plugin_path)))
{
    Write-Output "Creating plugin directory..."
    mkdir $plugin_path
}
Set-Location $origin
Copy-Item ".\target\wasm32-wasi\release\lapce-go.wasm" -Destination ("{0}\.lapce\plugins\lapce-go\" -f $target)
Copy-Item ".\plugin.toml" -Destination ("{0}\.lapce\plugins\lapce-go\" -f $target)
Write-Output "Done deploying."
