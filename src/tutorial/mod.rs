use select::select_fn;


mod select;
mod watch;
mod timer;
mod sync;
mod channel;
mod spawn;
pub async fn mod_fn(){
    select_fn().await;
}