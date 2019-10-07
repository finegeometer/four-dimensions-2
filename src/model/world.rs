use crate::render::Vertex;

// #[derive(Default)]
// pub struct World;

// impl World {
//     pub fn triangles(&self) -> Vec<Vertex> {
//         let iter = ground(-1.5)
//             .chain(tree(nalgebra::Vector4::new(-1.5, 0., 0., -5.)))
//             .chain(tree(nalgebra::Vector4::new(-1.5, 0., 5., 2.)));
//         // .chain(tree(nalgebra::Vector4::new(-1.5, 3.5, 0., -10.)));
//         iter.collect()
//     }
// }

// #[rustfmt::skip]
// fn ground(height: f32) -> impl Iterator<Item = Vertex> {
//     vec![
//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10.,  10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10., -10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10., -10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10., -10., -10.), texcoord: [1., 1.] },

//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10., -10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10., -10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10., -10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10.,  10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10.,  10., -10.), texcoord: [1., 1.] },

//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10.,  10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10., -10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10., -10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10.,  10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height,  10., -10.,  10.), texcoord: [1., 1.] },
//         Vertex { pos: nalgebra::Vector4::new(height, -10., -10.,  10.), texcoord: [1., 1.] },
//     ]
//     .into_iter()
// }

// fn tree(position: nalgebra::Vector4<f32>) -> impl Iterator<Item = Vertex> {
//     icosahedral_group().flat_map(move |q| {
//         let mut m = q
//             .to_rotation_matrix()
//             .matrix()
//             .insert_row(0, 0.)
//             .insert_column(0, 0.);
//         m[(0, 0)] = 1.;

//         vec![
//             // Foliage
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(6., 0., 0., 0.),
//                 texcoord: [1., 1.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(1., 1. - PHI, PHI, 0.),
//                 texcoord: [0., 1.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(1., PHI - 1., PHI, 0.),
//                 texcoord: [0., 1.],
//             },
//             Vertex {
//                 pos: position
//                     + m * nalgebra::Vector4::new(1., 0., 0.6 * PHI + 0.2, 0.2 * PHI + 0.4),
//                 texcoord: [0., 1.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(1., 1. - PHI, PHI, 0.),
//                 texcoord: [0., 1.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(1., PHI - 1., PHI, 0.),
//                 texcoord: [0., 1.],
//             },
//             Vertex {
//                 pos: position
//                     + m * nalgebra::Vector4::new(1., 0., 0.6 * PHI + 0.2, 0.2 * PHI + 0.4),
//                 texcoord: [0., 1.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(1., 1. - PHI, PHI, 0.),
//                 texcoord: [0., 1.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(1., PHI - 1., PHI, 0.),
//                 texcoord: [0., 1.],
//             },
//             // Trunk
//             Vertex {
//                 pos: position
//                     + m * nalgebra::Vector4::new(0., 0., 0.15 * PHI + 0.05, 0.05 * PHI + 0.1),
//                 texcoord: [0., 0.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(0., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
//                 texcoord: [55. / 64., 0.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(0., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
//                 texcoord: [0., 55. / 64.],
//             },
//             Vertex {
//                 pos: position
//                     + m * nalgebra::Vector4::new(0., 0., 0.15 * PHI + 0.05, 0.05 * PHI + 0.1),
//                 texcoord: [0., 0.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(0., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
//                 texcoord: [55. / 64., 0.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(0., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
//                 texcoord: [0., 55. / 64.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(0., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
//                 texcoord: [55. / 64., 0.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(0., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
//                 texcoord: [0., 55. / 64.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(1., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
//                 texcoord: [0., 55. / 64.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(1., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
//                 texcoord: [0., 55. / 64.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(1., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
//                 texcoord: [55. / 64., 0.],
//             },
//             Vertex {
//                 pos: position + m * nalgebra::Vector4::new(0., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
//                 texcoord: [55. / 64., 0.],
//             },
//         ]
//         .into_iter()
//     })
// }

fn icosahedral_group() -> impl Iterator<Item = nalgebra::UnitQuaternion<f32>> {
    fn f(axis: [f32; 3], angle: f32) -> nalgebra::UnitQuaternion<f32> {
        nalgebra::UnitQuaternion::new(nalgebra::Vector3::from(axis).normalize() * angle)
    }

    let pentagons = &[
        [PHI, 1., 0.],
        [-PHI, 1., 0.],
        [0., PHI, 1.],
        [0., -PHI, 1.],
        [1., 0., PHI],
        [1., 0., -PHI],
    ];

    let triangles = &[
        [PHI - 1., PHI, 0.],
        [PHI - 1., -PHI, 0.],
        [0., PHI - 1., PHI],
        [0., PHI - 1., -PHI],
        [PHI, 0., PHI - 1.],
        [-PHI, 0., PHI - 1.],
        [1., 1., 1.],
        [1., 1., -1.],
        [1., -1., 1.],
        [1., -1., -1.],
    ];

    let rectangles = &[
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        [1., PHI, PHI - 1.],
        [-1., PHI, PHI - 1.],
        [1., -PHI, PHI - 1.],
        [-1., -PHI, PHI - 1.],
        [PHI, PHI - 1., 1.],
        [PHI, PHI - 1., -1.],
        [-PHI, PHI - 1., 1.],
        [-PHI, PHI - 1., -1.],
        [PHI - 1., 1., PHI],
        [PHI - 1., -1., PHI],
        [PHI - 1., 1., -PHI],
        [PHI - 1., -1., -PHI],
    ];

    use std::f32::consts::PI;
    std::iter::once(nalgebra::UnitQuaternion::identity())
        .chain(
            pentagons
                .iter()
                .flat_map(|v| (1..5).map(move |n| f(*v, n as f32 * 2. * PI / 5.))),
        )
        .chain(
            triangles
                .iter()
                .flat_map(|v| (1..3).map(move |n| f(*v, n as f32 * 2. * PI / 3.))),
        )
        .chain(rectangles.iter().map(|v| f(*v, PI)))
}

const PHI: f32 = 1.618_034;

/*














*/

use crate::render::{Renderable, Transform};

#[derive(Default)]
pub struct World;

impl World {
    pub fn to_renderable(&self) -> impl Renderable {
        Transform::translation(
            (
                Ground,
                (
                    Transform::translation(Tree, nalgebra::Vector4::new(0., 0., 0., -5.)),
                    Transform::translation(Tree, nalgebra::Vector4::new(0., 0., 0., -10.)),
                ),
            ),
            nalgebra::Vector4::new(-1.5, 0., 0., 0.),
        )
    }
}

struct Ground;

impl Renderable for Ground {
    #[rustfmt::skip]
    fn triangles(&self) -> Box<dyn Iterator<Item = Vertex>> {
        Box::new(vec![
            Vertex { pos: nalgebra::Vector4::new(0., -10., -10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10., -10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10.,  10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10., -10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10., -10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10., -10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10., -10., -10.), texcoord: [1., 1.] },

            Vertex { pos: nalgebra::Vector4::new(0., -10., -10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10., -10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10., -10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10., -10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10., -10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10., -10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10.,  10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10.,  10., -10.), texcoord: [1., 1.] },

            Vertex { pos: nalgebra::Vector4::new(0., -10., -10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10.,  10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10., -10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10., -10., -10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10., -10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10.,  10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0.,  10., -10.,  10.), texcoord: [1., 1.] },
            Vertex { pos: nalgebra::Vector4::new(0., -10., -10.,  10.), texcoord: [1., 1.] },
        ]
        .into_iter())
    }

    fn regions(&self) -> Box<dyn Iterator<Item = Vec<nalgebra::RowVector5<f32>>>> {
        Box::new(std::iter::once(vec![nalgebra::RowVector5::new(
            0., 0., 0., 0., -1.,
        )]))
    }
}

struct Tree;

impl Renderable for Tree {
    fn triangles(&self) -> Box<dyn Iterator<Item = Vertex>> {
        Box::new(icosahedral_group().flat_map(move |q| {
            let mut m = q
                .to_rotation_matrix()
                .matrix()
                .insert_row(0, 0.)
                .insert_column(0, 0.);
            m[(0, 0)] = 1.;

            vec![
                // Foliage
                Vertex {
                    pos: m * nalgebra::Vector4::new(6., 0., 0., 0.),
                    texcoord: [1., 1.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., 1. - PHI, PHI, 0.),
                    texcoord: [0., 1.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., PHI - 1., PHI, 0.),
                    texcoord: [0., 1.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., 0., 0.6 * PHI + 0.2, 0.2 * PHI + 0.4),
                    texcoord: [0., 1.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., 1. - PHI, PHI, 0.),
                    texcoord: [0., 1.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., PHI - 1., PHI, 0.),
                    texcoord: [0., 1.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., 0., 0.6 * PHI + 0.2, 0.2 * PHI + 0.4),
                    texcoord: [0., 1.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., 1. - PHI, PHI, 0.),
                    texcoord: [0., 1.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., PHI - 1., PHI, 0.),
                    texcoord: [0., 1.],
                },
                // Trunk
                Vertex {
                    pos: m * nalgebra::Vector4::new(0., 0., 0.15 * PHI + 0.05, 0.05 * PHI + 0.1),
                    texcoord: [0., 0.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(0., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
                    texcoord: [55. / 64., 0.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(0., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
                    texcoord: [0., 55. / 64.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(0., 0., 0.15 * PHI + 0.05, 0.05 * PHI + 0.1),
                    texcoord: [0., 0.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(0., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
                    texcoord: [55. / 64., 0.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(0., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
                    texcoord: [0., 55. / 64.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(0., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
                    texcoord: [55. / 64., 0.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(0., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
                    texcoord: [0., 55. / 64.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
                    texcoord: [0., 55. / 64.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., 0.25 * PHI - 0.25, 0.25 * PHI, 0.),
                    texcoord: [0., 55. / 64.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(1., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
                    texcoord: [55. / 64., 0.],
                },
                Vertex {
                    pos: m * nalgebra::Vector4::new(0., 0.25 - 0.25 * PHI, 0.25 * PHI, 0.),
                    texcoord: [55. / 64., 0.],
                },
            ]
            .into_iter()
        }))
    }

    fn regions(&self) -> Box<dyn Iterator<Item = Vec<nalgebra::RowVector5<f32>>>> {
        let foliage = vec![
            nalgebra::RowVector5::new(-1., 0., 0., 0., 1.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), PHI, 1., 0., -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), PHI, -1., 0., -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), -PHI, 1., 0., -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), -PHI, -1., 0., -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), 1., 0., PHI, -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), -1., 0., PHI, -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), 1., 0., -PHI, -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), -1., 0., -PHI, -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), 0., PHI, 1., -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), 0., PHI, -1., -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), 0., -PHI, 1., -0.2 * (PHI + 1.) * 6.),
            nalgebra::RowVector5::new(0.2 * (PHI + 1.), 0., -PHI, -1., -0.2 * (PHI + 1.) * 6.),
        ];
        let trunk = vec![
            nalgebra::RowVector5::new(-1., 0., 0., 0., 0.),
            nalgebra::RowVector5::new(1., 0., 0., 0., -1.),
            nalgebra::RowVector5::new(0., PHI, 1., 0., -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., PHI, -1., 0., -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., -PHI, 1., 0., -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., -PHI, -1., 0., -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., 1., 0., PHI, -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., -1., 0., PHI, -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., 1., 0., -PHI, -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., -1., 0., -PHI, -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., 0., PHI, 1., -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., 0., PHI, -1., -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., 0., -PHI, 1., -0.25 * (PHI + 1.)),
            nalgebra::RowVector5::new(0., 0., -PHI, -1., -0.25 * (PHI + 1.)),
        ];

        Box::new(std::iter::once(foliage).chain(std::iter::once(trunk)))
    }
}
