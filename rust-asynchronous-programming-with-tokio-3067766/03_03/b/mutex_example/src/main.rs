use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

async fn person(remote_arc: Arc<Mutex<i32>>, name: String, new_channel: i32 ) {
    // Request access to the remote
    let mut real_remote = remote_arc.lock().await;

    // Changing the channel
    *real_remote = new_channel;
    println!("{} changed the channel", name);
    println!("Watching channel {}", new_channel);

    sleep(Duration::from_secs(5)).await;




}
#[tokio::main]
async fn main() {
    let tv_channel = 10;
    let remote = Mutex::new(tv_channel);
    let remote_arc = Arc::new(remote);

    let mut task_handles = Vec::new();

    for (name, new_channel) in [("marcus", 11), ("Jovanna", 12), ("Carlo", 55)] {
        task_handles.push(tokio::spawn(person(
            remote_arc.clone(),
            name.to_string(),
            new_channel,
        )));
    }

    for handle in task_handles {
        handle.await.unwrap();
    }

}
