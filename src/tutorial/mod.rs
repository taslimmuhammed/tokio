use sync::sync_fn;

mod timer;
mod sync;

pub async fn mod_fn(){
 
 for i in 1..10{
    sync_fn(i).await;
 }
}