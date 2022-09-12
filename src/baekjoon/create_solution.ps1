$name=$args[0]
Write-Output "creating .\$name ..."
Copy-Item ".\snippet\snippet.rs" -Destination ".\$name"