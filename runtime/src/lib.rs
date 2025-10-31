use anyhow::Result;
use state::State;
use std::sync::{Arc, Mutex};
use wasmtime::{Engine, Instance, Linker, Module, Store};

pub struct Runtime<S: State + Send + 'static> {
    engine: Engine,
    state: Arc<Mutex<S>>,
}

impl<S: State + Send + 'static> Runtime<S> {
    pub fn new(state: S) -> Self {
        let engine = Engine::default();
        Self {
            engine,
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub fn deploy(&self, name: &str, wasm: &[u8]) -> Result<()> {
        let key = format!("contract:{name}").into_bytes();
        let mut st = self.state.lock().unwrap();
        st.set(key, wasm.to_vec());
        Ok(())
    }

    pub fn call_i32(&self, name: &str, method: &str, arg: i32) -> Result<i32> {
        let key = format!("contract:{name}").into_bytes();
        let st = self.state.lock().unwrap();
        let wasm = st
            .get(&key)
            .ok_or_else(|| anyhow::anyhow!("contract not found: {name}"))?;
        drop(st);

        let module = Module::from_binary(&self.engine, &wasm)?;
        let mut store = Store::new(&self.engine, ());
        let linker = Linker::new(&self.engine);
        let instance: Instance = linker.instantiate(&mut store, &module)?;
        let func = instance.get_typed_func::<i32, i32>(&mut store, method)?;
        let result = func.call(&mut store, arg)?;
        Ok(result)
    }
}
