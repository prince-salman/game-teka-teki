[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
Write-Host "Downloading Trunk pre-compiled binary..."
Invoke-WebRequest -Uri "https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-x86_64-pc-windows-msvc.zip" -OutFile "trunk.zip"
Expand-Archive -Path "trunk.zip" -DestinationPath "$env:USERPROFILE\.cargo\bin" -Force
Remove-Item "trunk.zip"
& "$env:USERPROFILE\.cargo\bin\trunk.exe" --version
