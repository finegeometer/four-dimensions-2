#![forbid(unsafe_code)]

mod utils;

mod fps;
mod model;
mod render;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::sync::mpsc;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().ok_or("no global `window` exists")?;

    let (sender, reciever) = mpsc::channel();

    let mut model = model::Model::init(window.clone(), sender.clone())?;

    #[allow(clippy::type_complexity)]
    let f: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut(f64)>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    let window_ = window.clone();
    let mut closure = move |time: f64| -> Result<(), JsValue> {
        window_
            .request_animation_frame(f.borrow().as_ref().unwrap_throw().as_ref().unchecked_ref())?;

        for msg in reciever.try_iter() {
            model.update(msg)?
        }

        model.frame(time)?;

        Ok(())
    };

    let closure = move |time: f64| {
        closure(time).unwrap_or_else(|err| wasm_bindgen::throw_val(err));
    };

    *g.borrow_mut() = Some(Closure::wrap(Box::new(closure)));
    window.request_animation_frame(g.borrow().as_ref().unwrap_throw().as_ref().unchecked_ref())?;

    Ok(())
}
