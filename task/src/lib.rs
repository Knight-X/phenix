use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, Wasi};

pub mod task {
  pub  struct wasm_executor {
    path: String,
    linker: crate::Linker,
    store: crate::Store,
  }


 impl wasm_executor {
  
    pub fn initialize(path: String) -> wasm_executor {
      let mut config = crate::Config::default();
      crate::Wasi::add_to_config(&mut config);
    
      let store = crate::Store::new(&crate::Engine::new(&config).unwrap());
    
      assert!(crate::Wasi::set_context(
        &store,
        crate::WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args().unwrap()
            .build().unwrap()
      )
      .is_ok());
      let linker = crate::Linker::new(&store);
        wasm_executor {
          path: path,
          linker: linker,
          store: store,
        }
     
    }

    pub fn run(&mut self) {
      let module = crate::Module::from_file(self.store.engine(), self.path.as_str()).unwrap();
      self.linker.module("", &module).unwrap();
      self.linker.get_default("").unwrap().typed::<(), ()>().unwrap().call(()).unwrap();
    }
  }
}

