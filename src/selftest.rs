use librqbit::*;
use sha1::{Sha1,Digest};
use std::time::Duration;
pub async fn run()->anyhow::Result<()> {
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos();
    let root=std::env::temp_dir().join(format!("thodtorrent-test-{stamp}"));
    let seed_dir=root.join("seed"); let download_dir=root.join("download");
    std::fs::create_dir_all(&seed_dir)?;std::fs::create_dir_all(&download_dir)?;
    let payload:Vec<u8>=(0..65536).map(|i|((i*31+i/97)%251) as u8).collect();
    std::fs::write(seed_dir.join("fixture.bin"),&payload)?;
    let mut torrent=b"d4:infod6:lengthi65536e4:name11:fixture.bin12:piece lengthi16384e6:pieces80:".to_vec();
    for chunk in payload.chunks(16384){torrent.extend_from_slice(&Sha1::digest(chunk));}torrent.extend_from_slice(b"ee");
    std::fs::write(root.join("fixture.torrent"),&torrent)?;
    let seed=Session::new_with_opts(seed_dir,SessionOptions{dht:None,disable_trackers:true,disable_local_service_discovery:true,listen:Some(ListenerOptions{listen_addr:"127.0.0.1:0".parse()?,ipv4_only:true,..Default::default()}),..Default::default()}).await?;
    let sh=seed.add_torrent(AddTorrent::from_bytes(torrent.clone()),Some(AddTorrentOptions{overwrite:true,..Default::default()})).await?.into_handle().unwrap();
    tokio::time::timeout(Duration::from_secs(20),sh.wait_until_completed()).await??;
    use tokio::io::{AsyncBufReadExt,AsyncWriteExt};
    use serde_json::{json,Value};
    let mut child=tokio::process::Command::new(std::env::current_exe()?).arg("--worker").env("THODHUB_DATA_DIR",root.join("state")).creation_flags(0x08000000).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).kill_on_drop(true).spawn()?;
    let mut input=child.stdin.take().unwrap();let mut lines=tokio::io::BufReader::new(child.stdout.take().unwrap()).lines();
    let request=json!({"method":"add","source":root.join("fixture.torrent"),"folder":download_dir,"initial_peers":[seed.listen_addr().unwrap().to_string()]});
    input.write_all(format!("{request}\n").as_bytes()).await?;input.flush().await?;
    tokio::time::timeout(Duration::from_secs(45),async {
        while let Some(line)=lines.next_line().await? {
            let value:Value=serde_json::from_str(&line)?;
            anyhow::ensure!(value["error"].is_null(),"Worker error: {}",value["error"]);
            if let Some(rows)=value["torrents"].as_array(){for row in rows {let s=&row["stats"];if s["finished"].as_bool()==Some(true) && s["state"]=="paused" {anyhow::ensure!(s["uploaded_bytes"].as_u64()==Some(0),"Unexpected upload");return Ok::<(),anyhow::Error>(());}}}
        }
        anyhow::bail!("Worker closed before download completion")
    }).await??;
    anyhow::ensure!(std::fs::read(download_dir.join("fixture.bin"))?==payload,"Downloaded payload mismatch");
    drop(input);anyhow::ensure!(tokio::time::timeout(Duration::from_secs(10),child.wait()).await??.success(),"Worker exit failed");seed.stop().await;
    std::fs::write(root.join("result.txt"),"PASS: local peer transfer; piece hashes; payload; stopped; zero upload")?;
    println!("PASS {}",root.display());
    Ok(())
}
