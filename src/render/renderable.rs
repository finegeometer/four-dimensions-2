use super::Vertex;

pub trait Renderable {
    fn triangles<'r>(&'r self) -> Box<dyn 'r + Iterator<Item = Vertex>>;
    fn regions<'r>(&'r self) -> Box<dyn 'r + Iterator<Item = Vec<nalgebra::RowVector5<f32>>>>;
    fn fragment_shader(&self) -> String {
        let mut out = String::new();

        out += r"#version 300 es

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

bool intersects_scene(vec4 pos, vec4 target) {
    vec2 minmax;
";

        for r in self.regions() {
            out += "    minmax = vec2(0., 0.999);\n";

            for h in r {
                out += &format!(
                    "    minmax = clip(minmax, pos, target, vec4({:.9}, {:.9}, {:.9}, {:.9}), {:.9});\n",
                    h[0], h[1], h[2], h[3], h[4]
                )
            }

            out += r"
    if (minmax.y > minmax.x) {
        return true;
    }
"
        }

        out += "
    return false;
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

";

        out
    }
}

pub struct Transform<R> {
    inner: R,
    transform: nalgebra::Matrix5<f32>,
    transform_inv: nalgebra::Matrix5<f32>,
}

impl<R> Transform<R> {
    pub fn translation(inner: R, vector: nalgebra::Vector4<f32>) -> Self {
        let t = nalgebra::Translation { vector };
        Self {
            inner,
            transform: t.to_homogeneous(),
            transform_inv: t.inverse().to_homogeneous(),
        }
    }
}

impl<R: Renderable> Renderable for Transform<R> {
    fn triangles<'r>(&'r self) -> Box<dyn 'r + Iterator<Item = Vertex>> {
        Box::new(self.inner.triangles().map(move |mut v| {
            let old_pos: nalgebra::Vector5<f32> = v.pos.fixed_resize(1.);
            let new_pos = self.transform * old_pos;
            v.pos = new_pos.fixed_rows::<nalgebra::U4>(0) / new_pos[4];
            v
        }))
    }

    fn regions<'r>(&'r self) -> Box<dyn 'r + Iterator<Item = Vec<nalgebra::RowVector5<f32>>>> {
        Box::new(self.inner.regions().map(move |mut r| {
            for h in r.iter_mut() {
                *h *= self.transform_inv;
            }
            r
        }))
    }
}

impl<R1: Renderable, R2: Renderable> Renderable for (R1, R2) {
    fn triangles<'r>(&'r self) -> Box<dyn 'r + Iterator<Item = Vertex>> {
        Box::new(self.0.triangles().chain(self.1.triangles()))
    }

    fn regions<'r>(&'r self) -> Box<dyn 'r + Iterator<Item = Vec<nalgebra::RowVector5<f32>>>> {
        Box::new(self.0.regions().chain(self.1.regions()))
    }
}
