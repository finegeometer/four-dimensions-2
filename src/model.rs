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

/// All of the information stored by the program
pub struct Model {
    keys: HashSet<String>,
    fps: Option<fps::FrameCounter>,
    //
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub canvas: web_sys::HtmlCanvasElement,
    pub info_box: web_sys::HtmlParagraphElement,

    pub slice_slider: web_sys::HtmlInputElement,
    //
    four_camera: FourCamera,
    three_camera: ThreeCamera,
    world: world::World,
    //
    render: Box<render::RenderFunction>,
}

impl Model {
    pub fn init() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("no global `window` exists")?;
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

        Ok(Model {
            keys: HashSet::new(),
            fps: None,
            //
            window,
            document,
            canvas,
            info_box,
            slice_slider,
            //
            four_camera: FourCamera::default(),
            three_camera: ThreeCamera::default(),
            world: World::default(),
            //
            render,
        })
    }

    pub fn view(&mut self) -> Result<(), JsValue> {
        (self.render)(
            &self.world.triangles(),
            self.four_camera.projection_matrix(),
            self.three_camera.projection_matrix(),
            self.four_camera.position,
            [1.0, 1.0, 0.1 * self.slice_slider.value_as_number() as f32],
        )?;

        Ok(())
    }

    pub fn update(&mut self, msg: Msg) -> Result<(), JsValue> {
        match msg {
            Msg::Click => {
                if !self.pointer_lock() {
                    self.canvas.request_pointer_lock();
                }
            }
            Msg::KeyDown(k) => {
                self.keys.insert(k.to_lowercase());
            }
            Msg::KeyUp(k) => {
                self.keys.remove(&k.to_lowercase());
            }
            Msg::MouseMove([x, y]) => {
                if self.pointer_lock() {
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
                if self.pointer_lock() {
                    self.four_camera.orientation.horizontal *= nalgebra::UnitQuaternion::new(
                        nalgebra::Vector3::new(-z as f32 * 1e-2, 0., 0.),
                    );
                }
            }
            Msg::Frame(time) => {
                let dt: f64;
                if let Some(fps) = &mut self.fps {
                    dt = fps.frame(time);

                    self.info_box.set_inner_text(&format!("{}", fps));

                    self.move_player(dt);

                    self.view()?;
                } else {
                    self.fps = Some(<fps::FrameCounter>::new(time));
                }
            }
            Msg::SliceSliderSlid => {}
        }
        Ok(())
    }

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

    fn pointer_lock(&self) -> bool {
        self.document.pointer_lock_element().is_some()
    }
}

pub enum Msg {
    Click,
    Frame(f64), // time in milliseconds, counted from the start of the program.
    MouseMove([i32; 2]),
    MouseWheel(f64),
    KeyDown(String),
    KeyUp(String),
    SliceSliderSlid,
}
