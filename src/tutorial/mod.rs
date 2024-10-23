use change::changel_fn;
use select::select_fn;


mod select;
mod watch;
mod timer;
mod sync;
mod channel;
mod spawn;
mod change;
pub async fn mod_fn(){
    changel_fn().await;
}