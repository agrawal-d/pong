mod utils;

use pong_logic::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once()
}

#[wasm_bindgen]
pub fn get_initial_state() -> JsValue {
    let s0 = State::new();
    JsValue::from_serde(&s0).unwrap()
}

#[wasm_bindgen]
pub fn get_next_state(s0: &JsValue, dp1: i32, dp2: i32) -> JsValue {
    let s0: State = s0.into_serde().unwrap();
    let s1 = s0.next_state(dp1, dp2);
    JsValue::from_serde(&s1).unwrap()
}
