use tutorial::mod_fn;


mod tutorial;

pub async fn lib_fn(){
    mod_fn().await;
}