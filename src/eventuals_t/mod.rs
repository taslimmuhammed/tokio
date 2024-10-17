use join::join_fn;

mod join;

pub async fn evetual_fn(){
    join_fn().await;
}