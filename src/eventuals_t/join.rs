use std::{borrow::Borrow, time::Duration};

use eventuals::{Eventual, EventualExt};
use tokio::{select, sync::watch, time::sleep};




pub async fn join_fn() {
    let (mut writer, reader) = Eventual::<i32>::new();
    writer.write(1);
    let j1 =  tokio::spawn(async move{
        println!("writing....");
        sleep(Duration::from_millis(1000)).await;
        writer.write(3);
        println!("writing complete");
        println!("writing....");
        sleep(Duration::from_millis(1000)).await;
        writer.write(6);
        println!("writing complete");
    });
    // let j2 = tokio::spawn(async move{
    //     loop{
    //          println!("reading....");
    //         let mut subscriber = reader.subscribe();
    //         let mut val=0;
    //         select! {
    //             Ok(x) = subscriber.next()=>{val=x},
    //         }
    //         println!("{val}");
    //     }
       
    // });
    let (sender, mut receiver) = watch::channel(0);
    let _p1 = reader.pipe(move |x|{
        let _ = sender.send(x);
    });
    loop {
        select! {
            Ok(_) = receiver.changed()=>{},
        }
        let val = *receiver.borrow();
        println!("values is {val}");
        
    }
    // j2.await;
}