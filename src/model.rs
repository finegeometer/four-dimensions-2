mod four_camera;
mod three_camera;
mod world;

use crate::{fps, render};
use core::f32::consts::FRAC_PI_2;
use four_camera::FourCamera;
use three_camera::ThreeCamera;
use world::World;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Model {
    sender: std::sync::mpsc::Sender<Msg>,
    keys: HashSet<String>,
    fps: Option<fps::FrameCounter>,
    render: Box<render::RenderFunction>,

    window: web_sys::Window,
    document: web_sys::Document,
    canvas: web_sys::HtmlCanvasElement,
    info_box: web_sys::HtmlParagraphElement,
    slice_slider: web_sys::HtmlInputElement,

    four_camera: FourCamera,
    three_camera: ThreeCamera,
    world: world::World,
}

pub enum Msg {
    Click,
    MouseMove([i32; 2]),
    MouseWheel(f64),
    KeyDown(String),
    KeyUp(String),
    SliceSliderSlid,
}

impl Model {
    pub fn init(
        window: web_sys::Window,
        sender: std::sync::mpsc::Sender<Msg>,
    ) -> Result<Self, JsValue> {
        let document = window
            .document()
            .ok_or("should have a document on window")?;
        let body = document.body().ok_or("document should have a body")?;

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        canvas.set_attribute("width", "800")?;
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

        let canvas_ = canvas.clone();
        let document_ = document.clone();
        crate::utils::event_listener(&sender, &canvas, "mousedown", move |_| {
            if document_.pointer_lock_element().is_none() {
                canvas_.request_pointer_lock();
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

        Ok(Self {
            sender,
            keys: HashSet::new(),
            fps: None,
            render,

            window,
            document,
            canvas,
            info_box,
            slice_slider,

            four_camera: FourCamera::default(),
            three_camera: ThreeCamera::default(),
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
        }
        Ok(())
    }

    pub fn frame(&mut self, time: f64) -> Result<(), JsValue> {
        let dt: f64;
        if let Some(fps) = &mut self.fps {
            dt = fps.frame(time);

            self.info_box.set_inner_text(&format!("{}", fps));

            self.move_player(dt);

            (self.render)(
                &self.world.triangles(),
                self.four_camera.projection_matrix(),
                self.three_camera.projection_matrix(),
                self.four_camera.position,
                [1.0, 1.0, 0.1 * self.slice_slider.value_as_number() as f32],
            )?;
        } else {
            self.fps = Some(<fps::FrameCounter>::new(time));
        }

        Ok(())
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
