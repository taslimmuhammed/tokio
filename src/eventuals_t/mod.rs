use join::join_fn;
use pipe::pipe_fn;

mod join;
mod pipe;
pub async fn evetual_fn(){
    // join_fn().await;
    let handle = pipe_fn().await;
    handle.await;
}