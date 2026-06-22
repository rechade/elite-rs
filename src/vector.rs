#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
// pub struct Matrix {
//     matrix: [Vector; 3],
// }
pub type Matrix = [Vector; 3];
pub const START_MATRIX: [Vector; 3] = [
    Vector {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    },
    Vector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    },
    Vector {
        x: 0.0,
        y: 0.0,
        z: -1.,
    },
];
fn set_init_matrix(mat: &mut Matrix) {
    for i in 0..3 {
        mat[i] = START_MATRIX[i];
    }
}
pub const START_VECTOR: Vector = Vector {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub fn vector_dot_product(first: &Vector, second: &Vector) -> f32 {
    return (first.x * second.x) + (first.y * second.y) + (first.z * second.z);
}
