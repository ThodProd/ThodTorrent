#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{io::{BufRead, Write}, path::PathBuf, process::{Command, Stdio}, sync::{Arc, Mutex, mpsc}, time::Duration};
use std::os::windows::process::CommandExt;
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
mod integration;
mod selftest;

fn data_dir() -> PathBuf {
    std::env::var_os("THODHUB_DATA_DIR").map(PathBuf::from).or_else(|| winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER).open_subkey("Software\\ThodTorrent").ok().and_then(|k|k.get_value::<String,_>("DataDir").ok()).map(PathBuf::from)).unwrap_or_else(|| PathBuf::from(std::env::var_os("LOCALAPPDATA").unwrap()).join("ThodHUB/Data/thodtorrent"))
}
#[derive(Serialize, Deserialize)]
struct Settings { folder: String, columns: Vec<bool>, dark: bool }
const COLUMNS:[&str;13]=["Имя","Прогресс","Размер","Скачано","Скачивание","Отдача","Осталось","Состояние","Папка","Отдано","Рейтинг","Частей","Хеш"];
impl Default for Settings {
    fn default() -> Self { Self { folder: PathBuf::from(std::env::var_os("USERPROFILE").unwrap()).join("Downloads").display().to_string(), columns: (0..13).map(|i|i<8).collect(), dark: false } }
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
    let options = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_icon(icon).with_inner_size([1100., 650.]).with_min_inner_size([720.,560.]), ..Default::default() };
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
    child: std::process::Child, input: Option<std::process::ChildStdin>, rx: mpsc::Receiver<Value>,
}
impl App {
    fn new(cc: &eframe::CreationContext<'_>, target: String) -> Self {
        let mut settings: Settings = std::fs::read(data_dir().join("settings.json")).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
        settings.columns.resize(COLUMNS.len(),false);settings.columns[0]=true;
        let mut fonts = egui::FontDefinitions::default();
        if let Ok(bytes) = std::fs::read("C:/Windows/Fonts/segoeui.ttf") { fonts.font_data.insert("Segoe".into(), egui::FontData::from_owned(bytes).into()); fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0,"Segoe".into()); }
        cc.egui_ctx.set_fonts(fonts);
        let _=std::fs::create_dir_all(data_dir().join("logs"));
        let log=std::fs::File::create(data_dir().join("logs/worker.log")).unwrap();
        let mut child = Command::new(std::env::current_exe().unwrap()).arg("--worker").creation_flags(0x08000000).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::from(log)).spawn().expect("Не удалось запустить движок");
        let input = child.stdin.take(); let output = child.stdout.take().unwrap(); let (tx,rx) = mpsc::channel();
        std::thread::spawn(move || { for line in std::io::BufReader::new(output).lines().map_while(Result::ok) { if let Ok(v) = serde_json::from_str(&line) { if tx.send(v).is_err() { break; } } } });
        Self { folder: settings.folder.clone(), settings, adding: !target.is_empty(), source: target, rows:vec![], error:String::new(), selected:None, confirm:None, details:None, notice:String::new(), child,input,rx }
    }
    fn send(&mut self, value: Value) { if let Some(input) = &mut self.input { if let Err(e) = writeln!(input,"{value}").and_then(|_| input.flush()) { self.error = format!("Движок недоступен: {e}"); } } }
    fn save(&mut self) { if let Err(e) = std::fs::write(data_dir().join("settings.json"), serde_json::to_vec_pretty(&self.settings).unwrap()) { self.error = format!("Не удалось сохранить настройки: {e}"); } }
}
fn size(v:f64) -> String { if v >= 1073741824. { format!("{:.2} ГБ",v/1073741824.) } else if v >=1048576. { format!("{:.1} МБ",v/1048576.) } else { format!("{:.1} КБ",v/1024.) } }
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if data_dir().join("runtime/stop").exists(){ctx.send_viewport_cmd(egui::ViewportCommand::Close);}
        if let Ok(Some(code))=self.child.try_wait(){self.error=format!("Движок завершился ({code}). Перезапустите приложение. Журнал: {}",data_dir().join("logs/worker.log").display());}
        if ctx.input(|i|i.key_pressed(egui::Key::Escape)){self.adding=false;self.confirm=None;}
        if let Ok(files)=std::fs::read_dir(data_dir().join("runtime/inbox")){ for entry in files.flatten(){if let Ok(bytes)=std::fs::read(entry.path()){if let Ok(target)=serde_json::from_slice::<String>(&bytes){if !self.adding {self.source=target;self.adding=!self.source.is_empty();ctx.send_viewport_cmd(egui::ViewportCommand::Focus);let _=std::fs::remove_file(entry.path());}}}}}
        while let Ok(v) = self.rx.try_recv() { if let Some(rows) = v["torrents"].as_array() { self.rows=rows.clone(); } if let Some(e) = v["error"].as_str() { self.error=e.into();self.notice.clear(); } if !v["details"].is_null(){self.details=Some(v["details"].clone());} if v["ok"].as_bool()==Some(true){self.notice="Операция выполнена".into();} }
        let mut visual=if self.settings.dark {egui::Visuals::dark()} else {egui::Visuals::light()};
        visual.panel_fill=if self.settings.dark{egui::Color32::from_rgb(28,29,32)}else{egui::Color32::WHITE};
        visual.selection.bg_fill=egui::Color32::from_rgb(54,118,232);ctx.set_visuals(visual);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ThodTorrent"); ui.add_space(12.);
            ui.horizontal_wrapped(|ui| {
                if ui.button("Добавить ссылку").clicked() { self.adding=true; }
                if ui.button("Открыть .torrent").clicked() { if let Some(p)=rfd::FileDialog::new().add_filter("Торрент", &["torrent"]).pick_file() { self.source=p.display().to_string();self.adding=true; } }
                if ui.button("Продолжить").clicked() { if let Some(id)=self.selected {self.send(json!({"method":"start","id":id}));} }
                if ui.button("Стоп").clicked() { if let Some(id)=self.selected {self.send(json!({"method":"pause","id":id}));} }
                ui.menu_button("Столбцы", |ui| { for (i,name) in COLUMNS.iter().enumerate().skip(1) { if ui.checkbox(&mut self.settings.columns[i], *name).changed(){self.save();} } });
                if ui.checkbox(&mut self.settings.dark,"Тёмная тема").changed(){ self.save(); }
                if ui.button("По умолчанию").clicked(){match integration::run(false){Ok(())=>{let _=Command::new("explorer.exe").arg("ms-settings:defaultapps").spawn();},Err(e)=>self.error=format!("Регистрация: {e:#}")}}
            });
            ui.add_space(10.); ui.label("По завершении — стоп. Отдача отключена."); ui.separator();
            if !self.notice.is_empty(){ui.label(&self.notice);}
            let titles=COLUMNS;
            let rows=self.rows.clone();
            egui::ScrollArea::both().show(ui,|ui| {
                egui::Grid::new("torrents").striped(true).min_col_width(85.).spacing([16.,12.]).show(ui, |ui| {
                    for (i,title) in titles.iter().enumerate() {if self.settings.columns[i] {ui.strong(*title);}} ui.end_row();
                    for row in rows {
                        let id=row["id"].as_u64().unwrap_or(0); let s=&row["stats"]; let total=s["total_bytes"].as_f64().unwrap_or(0.); let done=s["progress_bytes"].as_f64().unwrap_or(0.);
                        for i in 0..COLUMNS.len() { if !self.settings.columns[i] {continue;} match i {
                            0=> { let response=ui.selectable_label(self.selected==Some(id),row["name"].as_str().unwrap_or("Получение метаданных…")); if response.clicked(){self.selected=Some(id);}
                                response.context_menu(|ui| { for (label,method) in [("Продолжить","start"),("Остановить","pause"),("Убрать из списка","forget")] {if ui.button(label).clicked(){self.send(json!({"method":method,"id":id}));ui.close_menu();}}
                                    if ui.button("Сведения и файлы").clicked(){self.send(json!({"method":"details","id":id}));ui.close_menu();}
                                    if ui.button("Открыть папку").clicked(){let _=Command::new("explorer.exe").arg(row["output_folder"].as_str().unwrap_or("")).spawn();ui.close_menu();}
                                    if ui.button("Скопировать magnet-ссылку").clicked(){ctx.copy_text(format!("magnet:?xt=urn:btih:{}",row["info_hash"].as_str().unwrap_or("")));ui.close_menu();}
                                    if ui.button("Удалить вместе с файлами…").clicked(){self.confirm=Some(id);ui.close_menu();}
                                });
                            },
                            1=>{ui.add(egui::ProgressBar::new(if total>0.{(done/total) as f32}else{0.}).show_percentage().desired_width(130.));},
                            2=>{ui.label(size(total));},3=>{ui.label(size(done));},
                            4|5=>{let key=if i==4{"download_speed"}else{"upload_speed"}; let speed=s["live"][key]["mbps"].as_f64().unwrap_or(0.);ui.label(format!("{}/с",size(speed*1048576.)));},
                            6=>{ui.label(s["live"]["time_remaining"]["human_readable"].as_str().unwrap_or("—"));},
                            7=>{ui.label(if s["finished"].as_bool()==Some(true){"Завершено · стоп"}else{match s["state"].as_str().unwrap_or(""){"live"=>"Скачивание","paused"=>"Остановлено","error"=>"Ошибка",_=>"Проверка"}});},
                            8=>{ui.label(row["output_folder"].as_str().unwrap_or(""));},
                            9=>{ui.label(size(s["uploaded_bytes"].as_f64().unwrap_or(0.)));},
                            10=>{ui.label(format!("{:.2}",if done>0.{s["uploaded_bytes"].as_f64().unwrap_or(0.)/done}else{0.}));},
                            11=>{ui.label(row["total_pieces"].to_string());},
                            _=>{ui.label(row["info_hash"].as_str().unwrap_or(""));}
                        }} ui.end_row();
                    }
                });
            });
            if !self.error.is_empty() { ui.separator();ui.colored_label(egui::Color32::RED,"Ошибка операции");ui.label(&self.error);if ui.button("Скрыть ошибку").clicked(){self.error.clear();} }
        });
        if self.adding { egui::Window::new("Добавить загрузку").collapsible(false).resizable(true).show(ctx,|ui| {
            ui.label("Magnet, ссылка на .torrent или путь к файлу");ui.text_edit_singleline(&mut self.source);
            ui.label("Сохранить в папку");ui.horizontal(|ui| {ui.text_edit_singleline(&mut self.folder);if ui.button("Выбрать папку").clicked(){if let Some(p)=rfd::FileDialog::new().pick_folder(){self.folder=p.display().to_string();}}});
            if ui.button("Запомнить папку по умолчанию").clicked(){self.settings.folder=self.folder.clone();self.save();}
            ui.horizontal(|ui| {if ui.button("Скачать").clicked() && !self.source.trim().is_empty() && !self.folder.trim().is_empty(){self.send(json!({"method":"add","source":self.source.trim(),"folder":self.folder}));self.notice="Получение метаданных и подготовка загрузки…".into();self.source.clear();self.adding=false;}if ui.button("Отмена").clicked(){self.adding=false;}});
        }); }
        if let Some(details)=self.details.clone(){let mut open=true;egui::Window::new("Сведения и файлы").open(&mut open).show(ctx,|ui|{ui.label(details["name"].as_str().unwrap_or("Торрент"));ui.label(details["output_folder"].as_str().unwrap_or(""));egui::ScrollArea::vertical().max_height(350.).show(ui,|ui|{if let Some(files)=details["files"].as_array(){for file in files{ui.label(format!("{} · {}",file["name"].as_str().unwrap_or(""),size(file["length"].as_f64().unwrap_or(0.))));}}});});if !open{self.details=None;}}
        if let Some(id)=self.confirm { egui::Window::new("Удалить файлы безвозвратно?").collapsible(false).show(ctx,|ui| {ui.label(self.rows.iter().find(|r|r["id"].as_u64()==Some(id)).and_then(|r|r["name"].as_str()).unwrap_or("Торрент"));if ui.button("Удалить торрент и файлы").clicked(){self.send(json!({"method":"delete","id":id}));self.confirm=None;}if ui.button("Отмена").clicked(){self.confirm=None;} }); }
        ctx.request_repaint_after(Duration::from_millis(250));
    }
}
impl Drop for App { fn drop(&mut self) { self.input.take(); for _ in 0..30 { if self.child.try_wait().ok().flatten().is_some(){return;} std::thread::sleep(Duration::from_millis(100)); } let _=self.child.kill();let _=self.child.wait(); } }
