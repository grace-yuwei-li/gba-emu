use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct CpuDebugInfo {
    pub pc: u32,
}
