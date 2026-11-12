use winreg::{RegKey, enums::*};
use anyhow::Result;
const OWNER: &str = "ThodTorrent";
#[link(name="shell32")]
unsafe extern "system" {fn SHChangeNotify(event:i32,flags:u32,item1:*const std::ffi::c_void,item2:*const std::ffi::c_void);}
fn notify(){unsafe{SHChangeNotify(0x08000000,0,std::ptr::null(),std::ptr::null());}}
pub fn run(remove:bool) -> Result<()> {
    let root=RegKey::predef(HKEY_CURRENT_USER);
    let keys=["Software\\Classes\\ThodTorrent.File","Software\\Classes\\ThodTorrent.Magnet","Software\\Classes\\Applications\\thodtorrent.exe","Software\\ThodTorrent"];
    if remove {
        for key in keys {if let Ok(k)=root.open_subkey(key){if k.get_value::<String,_>("Owner").ok().as_deref()==Some(OWNER){drop(k);root.delete_subkey_all(key)?;}}}
        if let Ok(k)=root.open_subkey_with_flags("Software\\RegisteredApplications",KEY_WRITE){let _=k.delete_value(OWNER);}
        if let Ok(k)=root.open_subkey_with_flags("Software\\Classes\\.torrent\\OpenWithProgids",KEY_WRITE){let _=k.delete_value("ThodTorrent.File");}
        notify();return Ok(());
    }
    let exe=std::env::current_exe()?.display().to_string();
    for key in keys {let (k,_)=root.create_subkey(key)?;k.set_value("Owner",&OWNER)?;}
    root.create_subkey("Software\\ThodTorrent")?.0.set_value("DataDir",&crate::data_dir().display().to_string())?;
    for (key,label) in [(keys[0],"Торрент ThodTorrent"),(keys[1],"Magnet ThodTorrent"),(keys[2],"ThodTorrent")] {
        let (k,_)=root.create_subkey(key)?;k.set_value("",&label)?;k.set_value("FriendlyAppName",&OWNER)?;
        k.create_subkey("DefaultIcon")?.0.set_value("",&format!("\"{exe}\",0"))?;
        k.create_subkey("shell\\open\\command")?.0.set_value("",&format!("\"{exe}\" --target \"%1\""))?;
    }
    root.create_subkey(keys[1])?.0.set_value("URL Protocol",&"")?;
    root.create_subkey("Software\\Classes\\Applications\\thodtorrent.exe\\SupportedTypes")?.0.set_value(".torrent",&"")?;
    root.create_subkey("Software\\Classes\\.torrent\\OpenWithProgids")?.0.set_value("ThodTorrent.File",&"")?;
    let (cap,_)=root.create_subkey("Software\\ThodTorrent\\Capabilities")?;
    cap.set_value("ApplicationName",&OWNER)?;cap.set_value("ApplicationDescription",&"Скачивание торрентов и magnet-ссылок")?;
    cap.create_subkey("FileAssociations")?.0.set_value(".torrent",&"ThodTorrent.File")?;
    cap.create_subkey("UrlAssociations")?.0.set_value("magnet",&"ThodTorrent.Magnet")?;
    root.create_subkey("Software\\RegisteredApplications")?.0.set_value(OWNER,&"Software\\ThodTorrent\\Capabilities")?;
    notify();Ok(())
}
