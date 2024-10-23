use std::time::Duration;

use eventuals::{join, timer, Eventual, EventualExt};
use tokio::{sync::watch, time::sleep};

pub async fn pipe_fn() -> tokio::task::JoinHandle<()>{
    let (mut writer, reader) = Eventual::<i32>::new();
    let (tx,mut rx) = watch::channel(0);
    //map is also continues listener like pipe
    let _ = reader.pipe(move|v|{
        tx.send(v).unwrap();
    }).forever();
    tokio::spawn(async move{
        sleep(Duration::from_millis(1000)).await;
        writer.write(2);
        rx.changed().await.unwrap();
        println!("{}", rx.borrow().clone());
        sleep(Duration::from_millis(1000)).await;
        writer.write(3);
        rx.changed().await.unwrap();
        println!("{}", rx.borrow().clone());
    })
}