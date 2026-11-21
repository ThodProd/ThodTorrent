$ErrorActionPreference='Stop'
Add-Type -AssemblyName System.Drawing
$streams=@()
foreach($n in @(16,24,32,48,64,128,256)) {
 $bmp=[Drawing.Bitmap]::new($n,$n)
 $g=[Drawing.Graphics]::FromImage($bmp)
 $g.SmoothingMode='AntiAlias'
 $g.Clear([Drawing.Color]::Transparent)
 $g.ScaleTransform($n / 64.0, $n / 64.0)

 # Match the 64-unit geometry and colors in assets/icon.svg.
 $tile=[Drawing.Drawing2D.GraphicsPath]::new()
 $tile.AddArc(1,1,30,30,180,90)
 $tile.AddArc(33,1,30,30,270,90)
 $tile.AddArc(33,33,30,30,0,90)
 $tile.AddArc(1,33,30,30,90,90)
 $tile.CloseFigure()
 $gradient=[Drawing.Drawing2D.LinearGradientBrush]::new(
  [Drawing.RectangleF]::new(1,1,62,62),
  [Drawing.ColorTranslator]::FromHtml('#f1f6ff'),
  [Drawing.ColorTranslator]::FromHtml('#dce8fb'),
  [Drawing.Drawing2D.LinearGradientMode]::Vertical
 )
 $border=[Drawing.Pen]::new([Drawing.ColorTranslator]::FromHtml('#cbdaf0'),2)
 $g.FillPath($gradient,$tile)
 $g.DrawPath($border,$tile)

 $nodeColor=[Drawing.ColorTranslator]::FromHtml('#6085c0')
 $nodes=[Drawing.SolidBrush]::new($nodeColor)
 $branches=[Drawing.Pen]::new($nodeColor,3.5)
 $branches.LineJoin='Round'
 $g.DrawLines($branches,[Drawing.Point[]]@(
  [Drawing.Point]::new(15,25),[Drawing.Point]::new(24,34),
  [Drawing.Point]::new(40,34),[Drawing.Point]::new(49,25)
 ))
 $g.FillEllipse($nodes,11.5,21.5,7,7)
 $g.FillEllipse($nodes,45.5,21.5,7,7)

 $arrow=[Drawing.Pen]::new([Drawing.ColorTranslator]::FromHtml('#224e96'),4.8)
 $arrow.StartCap='Round'
 $arrow.EndCap='Round'
 $arrow.LineJoin='Round'
 $g.DrawLine($arrow,32,15,32,49)
 $g.DrawLines($arrow,[Drawing.Point[]]@(
  [Drawing.Point]::new(22,39),[Drawing.Point]::new(32,49),[Drawing.Point]::new(42,39)
 ))
 $ms=[IO.MemoryStream]::new();$bmp.Save($ms,[Drawing.Imaging.ImageFormat]::Png)
 $streams+=,@{Size=$n;Bytes=$ms.ToArray()}
 $ms.Dispose();$g.Dispose();$bmp.Dispose()
 $tile.Dispose();$gradient.Dispose();$border.Dispose()
 $nodes.Dispose();$branches.Dispose();$arrow.Dispose()
}
$path=Join-Path $PSScriptRoot '../assets/app.ico'
$file=[IO.File]::Create($path);$writer=[IO.BinaryWriter]::new($file)
$writer.Write([uint16]0);$writer.Write([uint16]1);$writer.Write([uint16]7)
$offset=6+16*7
foreach($entry in $streams){$dim=if($entry.Size -eq 256){0}else{$entry.Size};$writer.Write([byte]$dim);$writer.Write([byte]$dim);$writer.Write([byte]0);$writer.Write([byte]0);$writer.Write([uint16]1);$writer.Write([uint16]32);$writer.Write([uint32]$entry.Bytes.Length);$writer.Write([uint32]$offset);$offset+=$entry.Bytes.Length}
foreach($entry in $streams){$writer.Write([byte[]]$entry.Bytes)}
$writer.Dispose();$file.Dispose()
