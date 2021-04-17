use task::task;
use sched::schedule;

use std::net::SocketAddr;
use jsonrpsee_http_server::HttpServerBuilder;
use tokio::runtime::Builder;

async fn process() -> Result<(), Box<dyn std::error::Error>> {
        let server_addr = run_server().await;
        Ok(())
}


pub fn run_service() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    runtime.spawn(async move {
         process().await;
    });
 

    loop {
    }
}

async fn run_server() -> SocketAddr {
        let mut server = HttpServerBuilder::default().build("127.0.0.1:3000".parse().unwrap()).unwrap();
        server.register_method("run_wasm", |params| {
          let _params: Vec<String> = params.parse().unwrap();
          let _s: String = _params[0].clone();
          schedule::spwan(move |_p| {
               let mut wasminstance = task::wasm_executor::initialize(_p);
              wasminstance.run()
             }, _s);
        Ok("wasm running...")
       }).unwrap();
 
       let addr = server.local_addr().unwrap();
       println!("{}", addr);
       tokio::spawn(async move { server.start().await.unwrap() });
       addr
}


