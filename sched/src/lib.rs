#[macro_use]
extern crate may;


pub mod schedule {

  pub fn spwan<F, T>(func: F, _s: String)
   where 
       F: Fn(String) -> T + Send + 'static,
       T: Send + 'static   
  {
    may::config().set_stack_size(0x2000);
    may::config().set_workers(2);
    go!(move || {
        func(_s)
    });
  }
}


