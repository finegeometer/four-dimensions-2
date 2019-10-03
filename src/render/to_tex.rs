use super::program::Program;
use crate::utils::as_f32_array;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

type GL = web_sys::WebGl2RenderingContext;

const VERTEX_SHADER: &str = r#"#version 300 es

in vec4 pos;
in vec2 texcoord;

out vec4 vpos;
out vec2 vtexcoord;
out vec4 vdata;

uniform mat4 four_camera_a;
uniform vec4 four_camera_b;

uniform mat4 three_camera;

void main() {
    vpos = pos;
    vtexcoord = texcoord;

    vdata = four_camera_a * pos + four_camera_b;

    gl_Position = three_camera * vdata.yxzw;
}

"#;

const FRAGMENT_SHADER: &str = r#"#version 300 es

precision mediump float;

in vec4 vpos;
in vec2 vtexcoord;
in vec4 vdata;

out vec4 color;

uniform vec4 four_camera_pos;
uniform sampler2D tex;
uniform vec3 three_screen_size;

vec2 clip(vec2 minmax, vec4 pos, vec4 target, vec4 abcd, float e) {
    float x = dot(abcd, pos) + e;
    float y = dot(abcd, target) + e;

    if (x > y) {
        minmax.x = max(minmax.x, x/(x-y));
    } else {
        minmax.y = min(minmax.y, x/(x-y));
    }

    return minmax;
}

bool intersects_foliage(vec4 pos, vec4 target) {
    pos.x -= 6.;
    target.x -= 6.;
    vec2 minmax = vec2(0., 0.999);

    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(-1., 0., 0., 0.), -5.);

    minmax = clip(minmax, pos, target, vec4(0.2 * 2.618034, 1.618034, 1., 0.), 0.);
    minmax = clip(minmax, pos, target, vec4(0.2 * 2.618034, 1.618034, -1., 0.), 0.);
    minmax = clip(minmax, pos, target, vec4(0.2 * 2.618034, -1.618034, 1., 0.), 0.);
    minmax = clip(minmax, pos, target, vec4(0.2 * 2.618034, -1.618034, -1., 0.), 0.);

    minmax = clip(minmax, pos.xzwy, target.xzwy, vec4(0.2 * 2.618034, 1.618034, 1., 0.), 0.);
    minmax = clip(minmax, pos.xzwy, target.xzwy, vec4(0.2 * 2.618034, 1.618034, -1., 0.), 0.);
    minmax = clip(minmax, pos.xzwy, target.xzwy, vec4(0.2 * 2.618034, -1.618034, 1., 0.), 0.);
    minmax = clip(minmax, pos.xzwy, target.xzwy, vec4(0.2 * 2.618034, -1.618034, -1., 0.), 0.);

    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(0.2 * 2.618034, 1.618034, 1., 0.), 0.);
    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(0.2 * 2.618034, 1.618034, -1., 0.), 0.);
    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(0.2 * 2.618034, -1.618034, 1., 0.), 0.);
    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(0.2 * 2.618034, -1.618034, -1., 0.), 0.);

    return minmax.y > minmax.x;
}

bool intersects_trunk(vec4 pos, vec4 target) {
    vec2 minmax = vec2(0., 0.999);

    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(-1., 0., 0., 0.), 0.);
    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(1., 0., 0., 0.), -1.);

    minmax = clip(minmax, pos, target, vec4(0., 1.618034, 1., 0.), -0.25 * 2.618034);
    minmax = clip(minmax, pos, target, vec4(0., 1.618034, -1., 0.), -0.25 * 2.618034);
    minmax = clip(minmax, pos, target, vec4(0., -1.618034, 1., 0.), -0.25 * 2.618034);
    minmax = clip(minmax, pos, target, vec4(0., -1.618034, -1., 0.), -0.25 * 2.618034);

    minmax = clip(minmax, pos.xzwy, target.xzwy, vec4(0., 1.618034, 1., 0.), -0.25 * 2.618034);
    minmax = clip(minmax, pos.xzwy, target.xzwy, vec4(0., 1.618034, -1., 0.), -0.25 * 2.618034);
    minmax = clip(minmax, pos.xzwy, target.xzwy, vec4(0., -1.618034, 1., 0.), -0.25 * 2.618034);
    minmax = clip(minmax, pos.xzwy, target.xzwy, vec4(0., -1.618034, -1., 0.), -0.25 * 2.618034);

    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(0., 1.618034, 1., 0.), -0.25 * 2.618034);
    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(0., 1.618034, -1., 0.), -0.25 * 2.618034);
    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(0., -1.618034, 1., 0.), -0.25 * 2.618034);
    minmax = clip(minmax, pos.xwyz, target.xwyz, vec4(0., -1.618034, -1., 0.), -0.25 * 2.618034);

    return minmax.y > minmax.x;
}

bool intersects_tree(vec4 pos, vec4 target) {
    return intersects_foliage(pos, target) || intersects_trunk(pos, target) || target.x < -1.5;
}

bool intersects_scene(vec4 pos, vec4 target) {
    vec4 tree1 = vec4(-1.5, 0., 0., -5.);
    vec4 tree2 = vec4(-1.5, 0., 5., 2.);
    return intersects_tree(pos - tree1, target - tree1) || intersects_tree(pos - tree2, target - tree2);
}

void main() {

    vec3 data = vdata.xyz / vdata.w;

    if (abs(data.x) > three_screen_size.x || abs(data.y) > three_screen_size.y || abs(data.z) > three_screen_size.z || abs(vdata.w) < 0.) {
        // Outside three-screen, so invisible.
        color = vec4(0.);
    } else if (intersects_scene(four_camera_pos, vpos)) {
        // Occluded, so invisible.
        color = vec4(0.);
    } else {
        color = texture(tex, vtexcoord) / 5.0;
    }
}

"#;

pub struct Vertex {
    pub pos: nalgebra::Vector4<f32>,
    pub texcoord: [f32; 2],
}

impl Vertex {
    fn iter(&self) -> impl Iterator<Item = &f32> {
        self.pos.iter().chain(self.texcoord.iter())
    }
}

pub type RenderFunction = dyn Fn(Uniforms) -> Result<(), JsValue>;

pub struct Uniforms {
    pub vertices: Vec<Vertex>,
    pub four_camera: nalgebra::Matrix4x5<f32>,
    pub four_camera_pos: nalgebra::Vector4<f32>,
    pub three_cameras: [nalgebra::Matrix4<f32>; 2],
    pub three_screen_size: [f32; 3],
}

pub fn make_fn(
    gl: Rc<GL>,
    render_texture: &web_sys::WebGlTexture,
) -> Result<Box<RenderFunction>, JsValue> {
    let program = Program::new(Rc::clone(&gl), VERTEX_SHADER, FRAGMENT_SHADER)?;

    let pos_loc = program.attribute("pos")?;
    let texcoord_loc = program.attribute("texcoord")?;
    let four_camera_a_loc = program.uniform("four_camera_a")?;
    let four_camera_b_loc = program.uniform("four_camera_b")?;
    let three_camera_loc = program.uniform("three_camera")?;
    let four_camera_pos_loc = program.uniform("four_camera_pos")?;
    let three_screen_size_loc = program.uniform("three_screen_size")?;
    let texture_loc = program.uniform("tex")?;

    let vao = gl
        .create_vertex_array()
        .ok_or("create_vertex_array failed")?;
    gl.bind_vertex_array(Some(&vao));

    let vertex_buffer = gl.create_buffer().ok_or("create_buffer failed")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.enable_vertex_attrib_array(pos_loc);
    gl.vertex_attrib_pointer_with_i32(pos_loc, 4, GL::FLOAT, false, 6 * 4, 0);
    gl.enable_vertex_attrib_array(texcoord_loc);
    gl.vertex_attrib_pointer_with_i32(texcoord_loc, 2, GL::FLOAT, false, 6 * 4, 4 * 4);

    let framebuffer = gl.create_framebuffer().ok_or("create_framebuffer failed")?;
    gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&framebuffer));
    gl.framebuffer_texture_2d(
        GL::FRAMEBUFFER,
        GL::COLOR_ATTACHMENT0,
        GL::TEXTURE_2D,
        Some(render_texture),
        0,
    );

    let texture = gl.create_texture().ok_or("create_texture failed")?;
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        0,                 // level
        GL::RGBA as i32,   // internal_format
        64,                // width
        64,                // height
        0,                 // border
        GL::RGBA,          // format
        GL::UNSIGNED_BYTE, // type
        Some(include_bytes!("../../resources/texture")),
    )?;
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

    let render: Box<RenderFunction> = Box::new(move |uniforms| {
        let data: Vec<f32> = uniforms
            .vertices
            .iter()
            .flat_map(|v| v.iter())
            .copied()
            .collect();

        gl.bind_framebuffer(GL::FRAMEBUFFER, Some(&framebuffer));
        gl.bind_vertex_array(Some(&vao));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &as_f32_array(&data)?.into(),
            GL::STATIC_DRAW,
        );

        gl.clear_color(0., 0., 0., 1.);
        gl.clear(GL::COLOR_BUFFER_BIT);

        gl.use_program(Some(&program));
        gl.bind_vertex_array(Some(&vao));

        gl.uniform_matrix4fv_with_f32_array(
            Some(&four_camera_a_loc),
            false,
            &uniforms
                .four_camera
                .fixed_slice::<nalgebra::U4, nalgebra::U4>(0, 0)
                .into_iter()
                .copied()
                .collect::<Vec<_>>(),
        );

        gl.uniform4f(
            Some(&four_camera_b_loc),
            uniforms.four_camera[(0, 4)],
            uniforms.four_camera[(1, 4)],
            uniforms.four_camera[(2, 4)],
            uniforms.four_camera[(3, 4)],
        );

        gl.uniform4f(
            Some(&four_camera_pos_loc),
            uniforms.four_camera_pos[0],
            uniforms.four_camera_pos[1],
            uniforms.four_camera_pos[2],
            uniforms.four_camera_pos[3],
        );

        gl.uniform3f(
            Some(&three_screen_size_loc),
            uniforms.three_screen_size[0],
            uniforms.three_screen_size[1],
            uniforms.three_screen_size[2],
        );

        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
        gl.uniform1i(Some(&texture_loc), 0);

        gl.viewport(0, 0, 800, 800);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&three_camera_loc),
            false,
            &uniforms.three_cameras[0]
                .into_iter()
                .copied()
                .collect::<Vec<_>>(),
        );
        gl.draw_arrays(GL::TRIANGLES, 0, (data.len() / 6) as i32);

        gl.viewport(800, 0, 800, 800);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&three_camera_loc),
            false,
            &uniforms.three_cameras[1]
                .into_iter()
                .copied()
                .collect::<Vec<_>>(),
        );
        gl.draw_arrays(GL::TRIANGLES, 0, (data.len() / 6) as i32);

        Ok(())
    });

    Ok(render)
}
