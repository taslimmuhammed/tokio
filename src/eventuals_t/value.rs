use std::time::Duration;

use eventuals::Eventual;
use tokio::time::sleep;


pub async fn value_fn(){
    let (mut writer, reader) = Eventual::<i32>::new();
    let j1 =  tokio::spawn(async move{
        sleep(Duration::from_millis(1000)).await;
        writer.write(3);
        sleep(Duration::from_millis(1000)).await;
        writer.write(6);
        sleep(Duration::from_millis(1000)).await;
        writer.write(9);
    });
    tokio::spawn(async move{
        loop{
            println!("{}", reader.value().await.unwrap());
        }
    });
    j1.await;
}