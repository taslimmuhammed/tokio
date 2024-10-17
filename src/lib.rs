use eventuals_t::evetual_fn;
use tutorial::mod_fn;

mod eventuals_t;
mod tutorial;

pub async fn lib_fn(){
   // mod_fn().await;
   evetual_fn().await;
}