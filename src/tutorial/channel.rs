
pub async fn channel_fn(){
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    tokio::spawn( async move{
        for i in 0..10{
            tx.send(i).await.unwrap();
        }
    });
    while let Some(val) = rx.recv().await {
        println!(" Got {}", val)
    }

}