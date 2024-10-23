// tokio does not have join so we use select macro

use tokio::sync::watch;
use tokio::time::{sleep, Duration};

pub async fn select_fn() {
    let (tx1, mut rx1) = watch::channel(0);
    let (tx2, mut rx2) = watch::channel(0);

    // Simulate sending updates on both channels
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        tx1.send(1).unwrap();
        println!("tx1 send 1");
        sleep(Duration::from_secs(1)).await;
        tx1.send(11).unwrap();
        println!("tx1 send 11");
        sleep(Duration::from_secs(1)).await;
        tx1.send(111).unwrap();
        println!("tx1 send 111");
        sleep(Duration::from_secs(1)).await;
        tx1.send(1111).unwrap();
        println!("tx1 send 1111");
    });

    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        tx2.send(2).unwrap();
        println!("tx2 send 2");
        sleep(Duration::from_secs(4)).await;
        tx2.send(22).unwrap();
        println!("tx2 send 22");
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
