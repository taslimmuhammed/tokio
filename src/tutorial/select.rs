// tokio does not have join so we use select macro

use tokio::sync::watch;
use tokio::time::{sleep, Duration};

pub async fn select_fn() {
    let (tx1, mut rx1) = watch::channel(0);
    let (tx2, mut rx2) = watch::channel(0);

    // Simulate sending updates on both channels
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        tx1.send(10).unwrap();
    });

    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        tx2.send(20).unwrap();
    });

    // Listening to both receivers
    loop {
        tokio::select! {
            Ok(()) = rx1.changed() => {
                println!("rx1 received: {}", *rx1.borrow());
            }
            Ok(()) = rx2.changed() => {
                println!("rx2 received: {}", *rx2.borrow());
            }
            else => break
        }
    }
}
