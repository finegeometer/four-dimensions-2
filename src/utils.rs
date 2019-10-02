use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn as_f32_array(v: &[f32]) -> Result<js_sys::Float32Array, JsValue> {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()?
        .buffer();

    let location = v.as_ptr() as u32 / 4;

    Ok(js_sys::Float32Array::new(&memory_buffer).subarray(location, location + v.len() as u32))
}

#[allow(dead_code)]
pub fn log<T: core::fmt::Debug>(x: T) {
    web_sys::console::log_1(&format!("{:?}", x).into());
}

pub fn event_listener<Msg: 'static>(
    sender: &std::sync::mpsc::Sender<Msg>,
    target: &web_sys::EventTarget,
    event: &str,
    msg: impl Fn(web_sys::Event) -> Msg + 'static,
) -> Result<(), JsValue> {
    let sender = sender.clone();
    let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::wrap(Box::new(move |evt| {
        sender
            .send(msg(evt))
            .expect_throw("Should be unreachable, because the reciever should not be dropped.");
    }));
    target.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}
