use crate::{bindings::wasvy::ecs::app::HostSystemSet, host::WasmHost, runner::State};
use anyhow::{Result, bail};
use wasmtime::component::Resource;

#[derive(Default)]
pub struct WasmSystemSet {
    pub(crate) name: String,
}

impl HostSystemSet for WasmHost {
    fn new(&mut self, name: String) -> Result<Resource<WasmSystemSet>> {
        let State::RunSystem { table, .. } = self.access() else {
            bail!("SystemSet can only be instantiated in a setup function")
        };

        Ok(table.push(WasmSystemSet { name })?)
    }

    fn drop(&mut self, serialize: Resource<WasmSystemSet>) -> Result<()> {
        let _ = self.table().delete(serialize)?;
        Ok(())
    }
}
