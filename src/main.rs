use std::path::Path;
use std::time::Duration;
use notify::{RecursiveMode};
use notify_debouncer_mini::new_debouncer;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {

    let (tx, mut rx) = mpsc::channel(1);
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, move |res| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            tx.send(res).await.unwrap();
        });
    }).unwrap();

    debouncer
        .watcher()
        .watch(Path::new("."), RecursiveMode::Recursive)
        .unwrap();

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}