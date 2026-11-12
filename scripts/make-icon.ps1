$ErrorActionPreference='Stop'
Add-Type -AssemblyName System.Drawing
$streams=@()
foreach($n in @(16,24,32,48,64,128,256)) {
 $bmp=[Drawing.Bitmap]::new($n,$n)
 $g=[Drawing.Graphics]::FromImage($bmp)
 $g.SmoothingMode='AntiAlias'
 $g.Clear([Drawing.Color]::Transparent)
 $blue=[Drawing.SolidBrush]::new([Drawing.ColorTranslator]::FromHtml('#3676E8'))
 $g.FillEllipse($blue,0,0,$n,$n)
 $pen=[Drawing.Pen]::new([Drawing.Color]::White, [single]($n*0.09))
 $g.DrawLine($pen,[single]($n*.5),[single]($n*.2),[single]($n*.5),[single]($n*.65))
 $g.DrawLine($pen,[single]($n*.3),[single]($n*.46),[single]($n*.5),[single]($n*.67))
 $g.DrawLine($pen,[single]($n*.7),[single]($n*.46),[single]($n*.5),[single]($n*.67))
 $g.DrawLine($pen,[single]($n*.28),[single]($n*.8),[single]($n*.72),[single]($n*.8))
 $ms=[IO.MemoryStream]::new();$bmp.Save($ms,[Drawing.Imaging.ImageFormat]::Png)
 $streams+=,@{Size=$n;Bytes=$ms.ToArray()}
 $ms.Dispose();$g.Dispose();$bmp.Dispose();$blue.Dispose();$pen.Dispose()
}
$path=Join-Path $PSScriptRoot '../assets/app.ico'
$file=[IO.File]::Create($path);$writer=[IO.BinaryWriter]::new($file)
$writer.Write([uint16]0);$writer.Write([uint16]1);$writer.Write([uint16]7)
$offset=6+16*7
foreach($entry in $streams){$dim=if($entry.Size -eq 256){0}else{$entry.Size};$writer.Write([byte]$dim);$writer.Write([byte]$dim);$writer.Write([byte]0);$writer.Write([byte]0);$writer.Write([uint16]1);$writer.Write([uint16]32);$writer.Write([uint32]$entry.Bytes.Length);$writer.Write([uint32]$offset);$offset+=$entry.Bytes.Length}
foreach($entry in $streams){$writer.Write([byte[]]$entry.Bytes)}
$writer.Dispose();$file.Dispose()
