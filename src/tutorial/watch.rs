use tokio::sync::watch;
use tokio::time::{sleep, Duration};

pub async fn watch_fn(){
    // Create a new watch channel with an initial value
    let (tx, mut rx1) = watch::channel(0);

    // Clone the receiver for a second task
    let mut rx2 = rx1.clone();

    // Task 1: Wait for updates in rx1
    tokio::spawn(async move {
        while rx1.changed().await.is_ok() {
            println!("Task 1 received: {:?}", rx1.borrow().clone());
        }
    });

    // Task 2: Wait for updates in rx2
    tokio::spawn(async move {
        while rx2.changed().await.is_ok() {
            println!("Task 2 received: {:?}", *rx2.borrow());
        }
    });

    // Simulate sending updates
    for i in 1..5 {
        tx.send(i).unwrap();
        sleep(Duration::from_secs(1)).await;
    }
}