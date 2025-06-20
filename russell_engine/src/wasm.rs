use std::collections::HashMap;

use crate::{Assignments, Engine};
use russell_ast::ASTNode;
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

    fn parse(&mut self, input: &str) -> Result<ASTNode, String> {
        self.inner.parse(input).map_err(|e| format!("{:?}", e))
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
            .eval_str(input, &Assignments(assignments_map))
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub fn check_tautology(&mut self, input: &str) -> Result<bool, String> {
        let expr = self.parse(input)?;

        self.inner
            .check_tautology(&expr)
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub fn check_contradiction(&mut self, input: &str) -> Result<bool, String> {
        let expr = self.parse(input)?;

        self.inner
            .check_contradiction(&expr)
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub fn check_contingency(&mut self, input: &str) -> Result<bool, String> {
        let expr = self.parse(input)?;

        self.inner
            .check_contingency(&expr)
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub fn compute_truth_table(&mut self, input: &str) -> Result<JsValue, String> {
        let expr = self.parse(input)?;
        let variables = self.inner.collect_variables(&expr);
        let rows = self.inner.compute_assignments(variables);

        let table = js_sys::Array::new();

        for assignments in rows {
            let result = self
                .inner
                .eval(&expr, &assignments)
                .map_err(|e| format!("{:?}", e))?;

            let row = js_sys::Object::new();

            for (var, value) in &assignments.0 {
                js_sys::Reflect::set(
                    &row,
                    &JsValue::from_str(&var.to_string()),
                    &JsValue::from_bool(*value),
                )
                .unwrap();
            }

            js_sys::Reflect::set(
                &row,
                &JsValue::from_str("result"),
                &JsValue::from_bool(result),
            )
            .unwrap();

            table.push(&row);
        }

        Ok(table.into())
    }
}
