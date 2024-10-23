use std::{thread::sleep, time::Duration};



pub async fn changel_fn(){
    let (tx, mut rx) = tokio::sync::watch::channel(0);
    let h = tokio::spawn( async move{
        for i in 1..10{
            sleep(Duration::from_millis(1000));
            tx.send(i).unwrap();
        }
    });
    let mut rx1 = rx.clone();
    tokio::spawn(async move{
        while rx1.changed().await.is_ok() {
            println!(" Got {}", *rx1.borrow());
        } 
    });
    tokio::spawn(async move{
        loop {
                println!(" Got x {}", *rx.borrow());
                if rx.changed().await.is_err() {
                    break;
                }
        }
    });
    h.await;
    

}