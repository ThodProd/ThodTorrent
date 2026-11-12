#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{io::{BufRead, Write}, path::PathBuf, process::{Command, Stdio}, sync::{Arc, Mutex, mpsc}, time::Duration};
use std::os::windows::process::CommandExt;
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
mod integration;
mod selftest;
mod ui;

fn data_dir() -> PathBuf {
    std::env::var_os("THODHUB_DATA_DIR").map(PathBuf::from).or_else(|| winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER).open_subkey("Software\\ThodTorrent").ok().and_then(|k|k.get_value::<String,_>("DataDir").ok()).map(PathBuf::from)).unwrap_or_else(|| PathBuf::from(std::env::var_os("LOCALAPPDATA").unwrap()).join("ThodHUB/Data/thodtorrent"))
}
#[derive(Serialize, Deserialize)]
struct Settings { folder: String, columns: Vec<bool>, dark: bool }
const COLUMNS:[&str;13]=["Имя","Прогресс","Размер","Скачано","Скачивание","Отдача","Осталось","Состояние","Папка","Отдано","Рейтинг","Частей","Хеш"];
impl Default for Settings {
    fn default() -> Self { Self { folder: PathBuf::from(std::env::var_os("USERPROFILE").unwrap()).join("Downloads").display().to_string(), columns: COLUMNS.iter().enumerate().map(|(i,_)| matches!(i,0|1|2|3|4|6)).collect(), dark: false } }
}
fn main() {if let Err(e)=run(){eprintln!("{e:#}");if !std::env::args().any(|a|a.starts_with("--")){rfd::MessageDialog::new().set_title("ThodTorrent — ошибка").set_description(format!("{e:#}")).set_level(rfd::MessageLevel::Error).show();}std::process::exit(1);}}
fn run() -> anyhow::Result<()> {
    std::fs::create_dir_all(data_dir())?;
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|a|a=="--self-test") {return tokio::runtime::Runtime::new()?.block_on(selftest::run());}
    if args.iter().any(|a| a == "--worker") { return tokio::runtime::Runtime::new()?.block_on(worker()); }
    if args.iter().any(|a| a == "--register-integrations") { return integration::run(false); }
    if args.iter().any(|a| a == "--cleanup") { return integration::run(true); }
    if args.iter().any(|a| a == "--default-app") { integration::run(false)?; Command::new("explorer.exe").arg("ms-settings:defaultapps").spawn()?; return Ok(()); }
    let target = args.iter().find(|a| !a.starts_with("--")).cloned().unwrap_or_default();
    let lock=std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(data_dir().join("instance.lock"))?;
    std::fs::create_dir_all(data_dir().join("runtime/inbox"))?;
    if args.iter().any(|a|a=="--stop-background"){
        if fs2::FileExt::try_lock_exclusive(&lock).is_ok(){return Ok(());}
        std::fs::write(data_dir().join("runtime/stop"),b"stop")?;
        for _ in 0..350{std::thread::sleep(Duration::from_millis(100));if fs2::FileExt::try_lock_exclusive(&lock).is_ok(){return Ok(());}}
        anyhow::bail!("Не удалось завершить ThodTorrent за 35 секунд");
    }
    if fs2::FileExt::try_lock_exclusive(&lock).is_err() {
        let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos();
        std::fs::write(data_dir().join(format!("runtime/inbox/{stamp}.json")),serde_json::to_vec(&target)?)?;
        return Ok(());
    }
    let icon=image::load_from_memory(include_bytes!("../assets/app.ico"))?.into_rgba8();
    let _=std::fs::remove_file(data_dir().join("runtime/stop"));
    let icon=egui::IconData{width:icon.width(),height:icon.height(),rgba:icon.into_raw()};
    let inner_size = if std::env::var("THODTORRENT_RENDER_CHECK").is_ok() {
        std::env::var("THODTORRENT_RENDER_SIZE").ok().and_then(|s| {
            let (w, h) = s.split_once('x')?;
            Some([w.trim().parse().ok()?, h.trim().parse().ok()?])
        }).unwrap_or([1160., 720.])
    } else { [1160., 720.] };
    let options = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_icon(icon).with_inner_size(inner_size).with_min_inner_size([780., 580.]), ..Default::default() };
    let result=eframe::run_native("ThodTorrent", options, Box::new(move |cc| Ok(Box::new(App::new(cc, target))))).map_err(|e| anyhow::anyhow!("{e}"));
    drop(lock);result
}
async fn worker() -> anyhow::Result<()> {
    use librqbit::*;
    let session = Session::new_with_opts(data_dir().join("downloads"), SessionOptions {
        persistence: Some(SessionPersistenceConfig::Json { folder: Some(data_dir().join("session")) }),
        dht: Some(DhtSessionConfig { persistence: None, ..Default::default() }),
        disable_upload: true,
        ..Default::default()
    }).await?;
    let api = Arc::new(Api::new(session.clone(), None));
    let output = Arc::new(Mutex::new(std::io::stdout()));
    let monitor = api.clone(); let out = output.clone();
    tokio::spawn(async move {
        loop {
            let list = monitor.api_torrent_list_ext(librqbit::api::ApiTorrentListOpts { with_stats: true });
            for t in &list.torrents { if let (Some(id), Some(s)) = (t.id, &t.stats) { if s.finished && matches!(s.state, TorrentStatsState::Live) { let _ = monitor.api_torrent_action_pause(id.into()).await; } } }
            if let Ok(mut w) = out.lock() { let _ = writeln!(w, "{}", json!({"torrents":list.torrents})); let _ = w.flush(); }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    std::thread::spawn(move || { for line in std::io::stdin().lock().lines() { match line { Ok(s) => { if tx.send(s).is_err() { break; } }, Err(_) => break } } });
    while let Some(line) = rx.recv().await {
        let req: Value = match serde_json::from_str(&line) { Ok(v) => v, Err(_) => continue };
        let api=api.clone();let output=output.clone();
        tokio::spawn(async move {
        let result: anyhow::Result<()> = async {
            let id = req["id"].as_u64().unwrap_or(0) as usize;
            match req["method"].as_str().unwrap_or("") {
                "add" => {
                    let source = req["source"].as_str().unwrap_or("");
                    let add = if source.starts_with("magnet:") || source.starts_with("https://") || source.starts_with("http://") { AddTorrent::from_url(source) } else { AddTorrent::from_bytes(std::fs::read(source)?) };
                    api.api_add_torrent(add, Some(AddTorrentOptions { output_folder: req["folder"].as_str().map(str::to_owned), initial_peers:req["initial_peers"].as_array().map(|a|a.iter().filter_map(|v|v.as_str()?.parse().ok()).collect()), ..Default::default() })).await?;
                },
                "pause" => { api.api_torrent_action_pause(id.into()).await?; },
                "start" => { if !api.api_stats_v1(id.into())?.finished { api.api_torrent_action_start(id.into()).await?; } },
                "forget" => { api.api_torrent_action_forget(id.into()).await?; },
                "delete" => { api.api_torrent_action_delete(id.into()).await?; },
                "details" => {let details=api.api_torrent_details(id.into())?;if let Ok(mut w)=output.lock(){let _=writeln!(w,"{}",json!({"details":details}));let _=w.flush();}},
                _ => anyhow::bail!("Неизвестная команда"),
            }
            Ok(())
        }.await;
        let reply = match result { Ok(()) => json!({"ok":true}), Err(e) => json!({"error":format!("{e:#}")}) };
        if let Ok(mut w) = output.lock() { let _=writeln!(w,"{reply}"); let _=w.flush(); }
        });
    }
    session.stop().await;
    Ok(())
}
struct App {
    settings: Settings, source: String, folder: String, adding: bool,
    rows: Vec<Value>, error: String, selected: Option<u64>, confirm: Option<u64>,
    details: Option<Value>, notice: String,
    filter: usize, search: String, preferences: bool, remember_folder: bool,
    preview_frames: u32, focus_source: bool, focus_search: bool,
    icon_tex: egui::TextureHandle,
    child: std::process::Child, input: Option<std::process::ChildStdin>, rx: mpsc::Receiver<Value>,
}
impl App {
    fn new(cc: &eframe::CreationContext<'_>, target: String) -> Self {
        let mut settings: Settings = std::fs::read(data_dir().join("settings.json")).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
        settings.columns.resize(COLUMNS.len(),false);settings.columns[0]=true;
        let mut fonts = egui::FontDefinitions::default();
        let mut regular = false;
        if let Ok(bytes) = std::fs::read("C:/Windows/Fonts/segoeui.ttf") {
            fonts.font_data.insert("Segoe".into(), egui::FontData::from_owned(bytes).into());
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0,"Segoe".into());
            regular = true;
        }
        let bold_name = if let Ok(bytes) = std::fs::read("C:/Windows/Fonts/segoeuib.ttf") {
            fonts.font_data.insert("Segoe-Bold".into(), egui::FontData::from_owned(bytes).into());
            "Segoe-Bold"
        } else { "Segoe" };
        if regular {
            fonts.families.insert(egui::FontFamily::Name("Bold".into()), vec![bold_name.to_owned(), "Segoe".into()]);
        } else {
            let fallback = fonts.families.get(&egui::FontFamily::Proportional).cloned().unwrap_or_default();
            fonts.families.insert(egui::FontFamily::Name("Bold".into()), fallback);
        }
        cc.egui_ctx.set_fonts(fonts);
        let icon = image::load_from_memory(include_bytes!("../assets/app.ico")).expect("Иконка приложения повреждена").into_rgba8();
        let color = egui::ColorImage::from_rgba_unmultiplied([icon.width() as usize, icon.height() as usize], icon.as_raw());
        let icon_tex = cc.egui_ctx.load_texture("app-icon", color, egui::TextureOptions::LINEAR);
        let _=std::fs::create_dir_all(data_dir().join("logs"));
        let log=std::fs::File::create(data_dir().join("logs/worker.log")).unwrap();
        let mut child = Command::new(std::env::current_exe().unwrap()).arg("--worker").creation_flags(0x08000000).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::from(log)).spawn().expect("Не удалось запустить движок");
        let input = child.stdin.take(); let output = child.stdout.take().unwrap(); let (tx,rx) = mpsc::channel();
        std::thread::spawn(move || { for line in std::io::BufReader::new(output).lines().map_while(Result::ok) { if let Ok(v) = serde_json::from_str(&line) { if tx.send(v).is_err() { break; } } } });
        let adding = !target.is_empty();
        Self { folder: settings.folder.clone(), settings, adding, source: target, rows:vec![], error:String::new(), selected:None, confirm:None, details:None, notice:String::new(), filter:0,search:String::new(),preferences:false,remember_folder:false,preview_frames:0,focus_source:adding,focus_search:false,icon_tex,child,input,rx }
    }
    fn send(&mut self, value: Value) { if let Some(input) = &mut self.input { if let Err(e) = writeln!(input,"{value}").and_then(|_| input.flush()) { self.error = format!("Движок недоступен: {e}"); } } }
    fn save(&mut self) { if let Err(e) = std::fs::write(data_dir().join("settings.json"), serde_json::to_vec_pretty(&self.settings).unwrap()) { self.error = format!("Не удалось сохранить настройки: {e}"); } }
}
fn size(v:f64) -> String { let s=if v >= 1073741824. { format!("{:.2} ГБ",v/1073741824.) } else if v >=1048576. { format!("{:.1} МБ",v/1048576.) } else { format!("{:.1} КБ",v/1024.) }; s.replace('.',",") }
impl Drop for App { fn drop(&mut self) { self.input.take(); for _ in 0..30 { if self.child.try_wait().ok().flatten().is_some(){return;} std::thread::sleep(Duration::from_millis(100)); } let _=self.child.kill();let _=self.child.wait(); } }
