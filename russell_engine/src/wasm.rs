use std::collections::HashMap;

use crate::Engine;
use anyhow::Context;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    // #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct WasmEngine {
    inner: Engine,
}

#[wasm_bindgen]
impl WasmEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEngine {
        WasmEngine {
            inner: Engine::default(),
        }
    }

    #[wasm_bindgen]
    pub fn eval(&mut self, input: &str, assignments: JsValue) -> Result<bool, String> {
        let assignments_map =
            serde_wasm_bindgen::from_value::<HashMap<String, bool>>(assignments).unwrap();

        let assignments_map = assignments_map
            .into_iter()
            .map(|(k, v)| (k.chars().next().unwrap(), v))
            .collect::<HashMap<char, bool>>();

        self.inner
            .eval_str(input.to_string(), &assignments_map)
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub fn check_tautology(&mut self, input: &str) -> Result<bool, String> {
        let parsed = self
            .inner
            .parse(input.to_string())
            .map_err(|e| format!("{:?}", e))?;

        self.inner
            .check_tautology(parsed)
            .map_err(|e| format!("{:?}", e))
    }
}
