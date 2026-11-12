$ErrorActionPreference='Stop'
Add-Type @'
using System;
using System.Runtime.InteropServices;
[ComImport, Guid("973810ae-9599-4b88-9e4d-6ee98c9552da"),InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IEnumAssocHandlers { [PreserveSig] int Next(uint count, [MarshalAs(UnmanagedType.Interface)] out IAssocHandler handler, out uint fetched); }
[ComImport, Guid("f04061ac-1659-4a3f-a954-775aa57fc083"),InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IAssocHandler { void GetName([MarshalAs(UnmanagedType.LPWStr)] out string name); void GetUIName([MarshalAs(UnmanagedType.LPWStr)] out string name); }
public static class AssocCheck {
 [DllImport("shell32.dll",CharSet=CharSet.Unicode)] static extern int SHAssocEnumHandlers(string extra,uint filter,out IEnumAssocHandlers handlers);
 public static string Find(){ IEnumAssocHandlers e;Marshal.ThrowExceptionForHR(SHAssocEnumHandlers(".torrent",0,out e)); try { IAssocHandler h;uint n;while(e.Next(1,out h,out n)==0 && n==1){try {string name,label;h.GetName(out name);h.GetUIName(out label);if(name.IndexOf("thodtorrent.exe",StringComparison.OrdinalIgnoreCase)>=0)return label+" | "+name;}finally{Marshal.ReleaseComObject(h);}}}finally{Marshal.ReleaseComObject(e);}throw new Exception("ThodTorrent missing from SHAssocEnumHandlers"); }
}
'@
[AssocCheck]::Find()
