Write-Host "Downloading rustup-init..."
Invoke-WebRequest -Uri "https://win.rustup.rs" -OutFile "rustup-init.exe"
Write-Host "Installing Rust non-interactively..."
.\rustup-init.exe -y -q
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
$env:Path += ";$env:USERPROFILE\.cargo\bin"
Write-Host "Adding WASM target..."
& "$env:USERPROFILE\.cargo\bin\rustup.exe" target add wasm32-unknown-unknown
Write-Host "Installing trunk..."
& "$env:USERPROFILE\.cargo\bin\cargo.exe" install --locked trunk
Write-Host "Installation Complete."
