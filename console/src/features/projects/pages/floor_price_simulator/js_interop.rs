#[cfg(not(feature = "server"))]
use wasm_bindgen::prelude::*;

#[cfg(not(feature = "server"))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = eval)]
    fn js_eval(script: &str) -> wasm_bindgen::JsValue;
}

#[cfg(not(feature = "server"))]
fn get_simulator_ns() -> Option<js_sys::Object> {
    let window = web_sys::window()?;
    let biyard = js_sys::Reflect::get(&window, &"biyard".into())
        .ok()
        .filter(|v| !v.is_undefined() && !v.is_null())?;
    js_sys::Reflect::get(&biyard, &"simulator".into())
        .ok()
        .filter(|v| !v.is_undefined() && !v.is_null())
        .map(|v| v.unchecked_into())
}

#[cfg(not(feature = "server"))]
pub(super) fn render_chart(canvas_id: &str, payload_json: &str) {
    use wasm_bindgen::JsCast;
    let Some(simulator) = get_simulator_ns() else {
        return;
    };
    let Ok(func) = js_sys::Reflect::get(&simulator, &"render_chart".into()) else {
        return;
    };
    let Ok(func) = func.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = func.call2(&simulator, &canvas_id.into(), &payload_json.into());
}

#[cfg(not(feature = "server"))]
pub(super) fn set_on_treasury_drag(cb: &Closure<dyn FnMut(i32, f64)>) {
    use wasm_bindgen::JsCast;
    let Some(simulator) = get_simulator_ns() else {
        return;
    };
    let Ok(func) = js_sys::Reflect::get(&simulator, &"set_on_treasury_drag".into()) else {
        return;
    };
    let Ok(func) = func.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = func.call1(&simulator, cb.as_ref().unchecked_ref());
}

#[cfg(not(feature = "server"))]
pub(super) fn destroy_chart(canvas_id: &str) {
    use wasm_bindgen::JsCast;
    let Some(simulator) = get_simulator_ns() else {
        return;
    };
    let Ok(func) = js_sys::Reflect::get(&simulator, &"destroy_chart".into()) else {
        return;
    };
    let Ok(func) = func.dyn_into::<js_sys::Function>() else {
        return;
    };
    let _ = func.call1(&simulator, &canvas_id.into());
}
