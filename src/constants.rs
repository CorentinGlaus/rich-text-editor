use crate::vertex::Vertex;

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [0.0, 1.0],
    }, // top-left
    Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 0.0],
    }, // bottom-left
    Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [1.0, 0.0],
    }, // bottom-right
    Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 1.0],
    }, // top-right
];

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
    2, 1, 0, // bottom-left triangle
    3, 2, 0, // top-right triangle
];
