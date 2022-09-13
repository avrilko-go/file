use std::path::Path;
use std::time::Duration;
use notify::{RecursiveMode};
use notify_debouncer_mini::new_debouncer;
use tokio::process::Command;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let map_files: Vec<&str> = vec!["crm.2345.cn", "fol.2345.net"];
    for file in map_files {
        tokio::spawn(begin_task(file));
    }

    loop {}
}

async fn begin_task(file: &str) {
    let (tx, mut rx) = mpsc::channel(1);
    let mut debouncer = new_debouncer(Duration::from_millis(500), None, move |res| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            tx.send(res).await.unwrap();
        });
    }).unwrap();
    let path = format!("/Users/avrilko/web/{}", file);
    debouncer
        .watcher()
        .watch(Path::new(&path), RecursiveMode::Recursive)
        .unwrap();

    while let Some(res) = rx.recv().await {
        match res {
            Ok(_) => command(file).await,
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

async fn command(file: &str) {
    Command::new("sync.sh").arg(file).env("PATH", ".").status().await.unwrap();
}