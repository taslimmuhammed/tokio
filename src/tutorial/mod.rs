use watch::watch_fn;


mod watch;
mod timer;
mod sync;
mod channel;
mod spawn;
pub async fn mod_fn(){
    watch_fn().await;
}