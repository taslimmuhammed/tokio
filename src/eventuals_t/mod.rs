use join::join_fn;
use pipe::pipe_fn;
use value::value_fn;

mod value;
mod join;
mod pipe;
pub async fn evetual_fn(){
    // join_fn().await;
    // let handle = pipe_fn().await;
    // handle.await;
    value_fn().await;
}