use spawn::spawn_fn;


mod timer;
mod sync;
mod channel;
mod spawn;
pub async fn mod_fn(){
    spawn_fn().await;
}