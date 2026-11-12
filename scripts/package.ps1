param([string]$HubRoot='D:\Projects\ThodPROD')
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
$stage=Join-Path $root 'stage'
New-Item -ItemType Directory -Path (Join-Path $stage 'app') -Force | Out-Null
Copy-Item (Join-Path $root 'target/release/thodtorrent.exe') (Join-Path $stage 'app/thodtorrent.exe')
Copy-Item (Join-Path $root 'target/release/libunwind.dll') (Join-Path $stage 'app/libunwind.dll')
Copy-Item (Join-Path $root 'hub/manifest.json') (Join-Path $stage 'manifest.json')
Copy-Item (Join-Path $root 'README.md') (Join-Path $stage 'README.md')
Copy-Item (Join-Path $root 'assets/app.ico') (Join-Path $stage 'app/app.ico')
$licenses=Join-Path $stage 'licenses'
New-Item -ItemType Directory -Path $licenses -Force | Out-Null
$registry=Join-Path $root '.tools/cargo/registry/src/index.crates.io-1949cf8c6b5b557f'
foreach($crate in Get-ChildItem $registry -Directory){
 $files=Get-ChildItem $crate.FullName -File | Where-Object {$_.Name -match '^(LICENSE|LICENCE|COPYING|NOTICE)'}
 if($files){$dest=Join-Path $licenses $crate.Name;New-Item -ItemType Directory -Path $dest -Force | Out-Null;foreach($file in $files){Copy-Item -LiteralPath $file.FullName -Destination $dest}}
}
Copy-Item (Join-Path $root 'assets/librqbit-LICENSE') $licenses
Copy-Item (Join-Path $HubRoot '.tools/llvm/llvm-mingw-20260616-ucrt-x86_64/LICENSE.TXT') (Join-Path $licenses 'llvm-mingw-LICENSE.txt')
& (Join-Path $HubRoot 'scripts/package-module.ps1') -PackageDirectory $stage -Repository 'ThodProd/ThodTorrent' -OutputDirectory (Join-Path $root 'dist') -Notes 'Первый выпуск: magnet и .torrent, выбор папки, настраиваемые столбцы, контекстное меню, остановка после загрузки и отключённая отдача.'
