use std::{borrow::Borrow, time::Duration};

use eventuals::{join, Eventual, EventualExt};
use tokio::{select, sync::watch, time::sleep};




pub async fn join_fn() {
    let (mut writer, reader) = Eventual::<i32>::new();
    let (mut writer2, reader2) = Eventual::<i32>::new();
    let j1 =  tokio::spawn(async move{
        println!("writing....1");
        sleep(Duration::from_millis(10000)).await;
        writer.write(3);
        println!("writing complete  1");
        println!("writing....");
        sleep(Duration::from_millis(10000)).await;
        writer.write(6);
        println!("writing complete 1");
        sleep(Duration::from_millis(10000)).await;
        writer.write(9);
        println!("writing complete 1");
        sleep(Duration::from_millis(10000)).await;
    });
    let j2 =  tokio::spawn(async move{
        println!("writing.... 2");
        sleep(Duration::from_millis(10000)).await;
        writer2.write(2);
        println!("writing complete 2");
        println!("writing.... 2");
        // sleep(Duration::from_millis(10000)).await;
        // writer2.write(4);
        // println!("writing complete 2");
        // sleep(Duration::from_millis(10000)).await;
        // writer2.write(8);
        // println!("writing complete 2");
    });
    let eve = join((reader, reader2)).map(move |(r1, r2)| {
        async  move{
            println!("joining...");
            (r1, r2)
        }
    });
    j1.await;
    j2.await;
    let _ = eve.pipe(move |x| {
        println!("{:?}",x);
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
    // let (sender, mut receiver) = watch::channel(0);
    // let _p1 = reader.pipe(move |x|{
    //     let _ = sender.send(x);
    // });
    // loop {
    //     select! {
    //         Ok(_) = receiver.changed()=>{},
    //     }
    //     let val = *receiver.borrow();
    //     println!("values is {val}");

    // }
    // j2.await;
}