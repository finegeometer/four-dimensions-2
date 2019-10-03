mod four_camera;
mod world;

use crate::{fps, render};
use core::f32::consts::FRAC_PI_2;
use four_camera::FourCamera;
use world::World;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Model {
    keys: HashSet<String>,
    fps: Option<fps::FrameCounter>,
    render: Box<render::RenderFunction>,

    window: web_sys::Window,
    document: web_sys::Document,
    info_box: web_sys::HtmlParagraphElement,
    slice_slider: web_sys::HtmlInputElement,
    vr_status: std::rc::Rc<std::cell::RefCell<VrStatus>>,

    four_camera: FourCamera,
    world: world::World,
}

#[derive(Clone)]
enum VrStatus {
    Searching,
    NotSupported,
    NotFound,
    Known(web_sys::VrDisplay),
    RequestedPresentation(web_sys::VrDisplay),
    Presenting(web_sys::VrDisplay),
}

pub enum Msg {
    Click,
    MouseMove([i32; 2]),
    MouseWheel(f64),
    KeyDown(String),
    KeyUp(String),
    SliceSliderSlid,

    GotVRDisplays(js_sys::Array),
    DisplayPresenting(web_sys::VrDisplay),
}

impl Model {
    pub fn init(
        window: web_sys::Window,
        sender: std::sync::mpsc::Sender<Msg>,
    ) -> Result<Self, JsValue> {
        web_sys::console::log_1(&"Testing: 3".into());

        let document = window
            .document()
            .ok_or("should have a document on window")?;
        let body = document.body().ok_or("document should have a body")?;

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        canvas.set_attribute("width", "1600")?;
        canvas.set_attribute("height", "800")?;
        body.append_child(&canvas)?;

        let info_box = document
            .create_element("p")?
            .dyn_into::<web_sys::HtmlParagraphElement>()?;
        body.append_child(&info_box)?;

        let slice_slider = document
            .create_element("input")?
            .dyn_into::<web_sys::HtmlInputElement>()?;

        slice_slider.set_type("range");
        slice_slider.set_min("1");
        slice_slider.set_max("10");
        slice_slider.set_value("10");
        body.append_child(&slice_slider)?;

        let render = render::make_fn(&canvas)?;

        let vr_status = std::rc::Rc::new(std::cell::RefCell::new(VrStatus::Searching));

        let canvas_ = canvas.clone();
        let document_ = document.clone();
        let vr_status_ = vr_status.clone();
        let sender_ = sender.clone();
        crate::utils::event_listener(&sender, &canvas, "mousedown", move |_| {
            if document_.pointer_lock_element().is_none() {
                canvas_.request_pointer_lock();
            }

            let temp = vr_status_.borrow().clone();
            if let VrStatus::Known(display) = temp {
                *vr_status_.borrow_mut() = VrStatus::RequestedPresentation(display.clone());

                let mut layer = web_sys::VrLayer::new();
                layer.source(Some(&canvas_));
                let layers = js_sys::Array::new();
                layers.set(0, layer.as_ref().clone());

                let display_ = display.clone();
                let sender_ = sender_.clone();
                let closure = Closure::once(move |_| {
                    sender_
                        .send(Msg::DisplayPresenting(display_))
                        .unwrap_throw()
                });
                display
                    .request_present(&layers)
                    .unwrap_throw()
                    .then(&closure);
                closure.forget();
            }

            Msg::Click
        })?;
        crate::utils::event_listener(&sender, &canvas, "mousemove", |evt| {
            let evt = evt.dyn_into::<web_sys::MouseEvent>().unwrap_throw();
            Msg::MouseMove([evt.movement_x(), evt.movement_y()])
        })?;
        crate::utils::event_listener(&sender, &canvas, "wheel", |evt| {
            let evt = evt.dyn_into::<web_sys::WheelEvent>().unwrap_throw();
            Msg::MouseWheel(evt.delta_y())
        })?;
        crate::utils::event_listener(&sender, &document, "keydown", |evt| {
            let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap_throw();
            Msg::KeyDown(evt.key())
        })?;
        crate::utils::event_listener(&sender, &document, "keyup", |evt| {
            let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap_throw();
            Msg::KeyUp(evt.key())
        })?;
        crate::utils::event_listener(&sender, &slice_slider, "input", |_| Msg::SliceSliderSlid)?;

        let navigator: web_sys::Navigator = window.navigator();

        let sender_ = sender.clone();
        if js_sys::Reflect::has(&navigator, &"getVRDisplays".into())? {
            let closure = Closure::once(move |vr_displays| {
                sender_
                    .send(Msg::GotVRDisplays(js_sys::Array::from(&vr_displays)))
                    .unwrap_throw();
            });
            navigator.get_vr_displays()?.then(&closure);
            closure.forget();
        } else {
            web_sys::console::error_1(
                &"WebVR is not supported by this browser, on this computer.".into(),
            );

            *vr_status.borrow_mut() = VrStatus::NotSupported;
        }

        Ok(Self {
            keys: HashSet::new(),
            fps: None,
            render,

            window,
            document,
            info_box,
            slice_slider,
            vr_status,

            four_camera: FourCamera::default(),
            world: World::default(),
        })
    }

    pub fn update(&mut self, msg: Msg) -> Result<(), JsValue> {
        match msg {
            Msg::Click => {}
            Msg::KeyDown(k) => {
                self.keys.insert(k.to_lowercase());
            }
            Msg::KeyUp(k) => {
                self.keys.remove(&k.to_lowercase());
            }
            Msg::MouseMove([x, y]) => {
                if self.document.pointer_lock_element().is_some() {
                    self.four_camera.orientation.horizontal *= nalgebra::UnitQuaternion::new(
                        nalgebra::Vector3::new(0., -x as f32 * 3e-3, 0.),
                    );
                    self.four_camera.orientation.vertical += y as f32 * 3e-3;
                    self.four_camera.orientation.vertical =
                        self.four_camera.orientation.vertical.min(FRAC_PI_2);
                    self.four_camera.orientation.vertical =
                        self.four_camera.orientation.vertical.max(-FRAC_PI_2);
                }
            }
            Msg::MouseWheel(z) => {
                if self.document.pointer_lock_element().is_some() {
                    self.four_camera.orientation.horizontal *= nalgebra::UnitQuaternion::new(
                        nalgebra::Vector3::new(-z as f32 * 1e-2, 0., 0.),
                    );
                }
            }
            Msg::SliceSliderSlid => {}
            Msg::GotVRDisplays(vr_displays) => {
                if vr_displays.length() == 0 {
                    *self.vr_status.borrow_mut() = VrStatus::NotFound;
                } else {
                    *self.vr_status.borrow_mut() = VrStatus::Known(vr_displays.get(0).dyn_into()?);
                }
            }
            Msg::DisplayPresenting(display) => {
                *self.vr_status.borrow_mut() = VrStatus::Presenting(display)
            }
        }
        Ok(())
    }

    pub fn frame(&mut self, time: f64) -> Result<(), JsValue> {
        let dt: f64;
        if let Some(fps) = &mut self.fps {
            dt = fps.frame(time);

            self.info_box.set_inner_text(&format!("{}", fps));

            self.move_player(dt);

            (self.render)(render::Uniforms {
                vertices: self.world.triangles(),
                four_camera: self.four_camera.projection_matrix(),
                four_camera_pos: self.four_camera.position,
                three_screen_size: [1., 1., 0.1 * self.slice_slider.value_as_number() as f32],
                three_cameras: if let VrStatus::Presenting(display) =
                    self.vr_status.borrow().clone()
                {
                    let frame_data = web_sys::VrFrameData::new()?;
                    display.get_frame_data(&frame_data);

                    [
                        (nalgebra::Matrix4::from_iterator(frame_data.left_projection_matrix()?)
                            * nalgebra::Matrix4::from_iterator(frame_data.left_view_matrix()?)),
                        (nalgebra::Matrix4::from_iterator(frame_data.right_projection_matrix()?)
                            * nalgebra::Matrix4::from_iterator(frame_data.right_view_matrix()?)),
                    ]
                } else {
                    [
                        nalgebra::Matrix4::new(
                            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 2.98, 0., 0., -1., 3.,
                        ),
                        nalgebra::Matrix4::new(
                            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 2.98, 0., 0., -1., 3.,
                        ),
                    ]
                },
            })?;

            if let VrStatus::Presenting(display) = self.vr_status.borrow().clone() {
                display.submit_frame();
            }
        } else {
            self.fps = Some(<fps::FrameCounter>::new(time));
        }

        Ok(())
    }

    pub fn request_animation_frame(&self, callback: &js_sys::Function) -> Result<i32, JsValue> {
        if let VrStatus::Presenting(display) = self.vr_status.borrow().clone() {
            display.request_animation_frame(callback)
        } else {
            self.window.request_animation_frame(callback)
        }
    }
}

impl Model {
    fn move_player(&mut self, dt: f64) {
        let m = self.four_camera.orientation.horizontal_to_mat() * dt as f32;
        if self.keys.contains(" ") {
            self.four_camera.position += m * nalgebra::Vector4::new(1., 0., 0., 0.);
        }
        if self.keys.contains("shift") {
            self.four_camera.position += m * nalgebra::Vector4::new(-1., 0., 0., 0.);
        }
        if self.keys.contains("w") {
            self.four_camera.position += m * nalgebra::Vector4::new(0., 0., 0., -1.);
        }
        if self.keys.contains("s") {
            self.four_camera.position += m * nalgebra::Vector4::new(0., 0., 0., 1.);
        }
        if self.keys.contains("d") {
            self.four_camera.position += m * nalgebra::Vector4::new(0., 1., 0., 0.);
        }
        if self.keys.contains("a") {
            self.four_camera.position += m * nalgebra::Vector4::new(0., -1., 0., 0.);
        }
        if self.keys.contains("q") {
            self.four_camera.position += m * nalgebra::Vector4::new(0., 0., 1., 0.);
        }
        if self.keys.contains("e") {
            self.four_camera.position += m * nalgebra::Vector4::new(0., 0., -1., 0.);
        }
    }
}
