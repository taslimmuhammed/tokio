pub async fn spawn_fn(){
    let mut i = 0;
    tokio::spawn( async move {
        i = 1;
    });
    tokio::spawn( async move {
        i = 2;
    });
    tokio::spawn( async move {
        i = 3;
    });
    tokio::spawn( async move {
        i = 4;
    });
    let res = tokio::spawn( async move {
        i = 5;
    });
    println!("last one is {}",i);
}