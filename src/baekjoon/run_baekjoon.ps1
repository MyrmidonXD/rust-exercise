$code=$args[0]
rustc $code -o out.exe
if ($LastExitCode -eq 0) {
    Get-Content ./snippet/sample_input.txt | ./out.exe
}