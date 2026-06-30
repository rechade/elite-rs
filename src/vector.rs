#[derive(Copy, Clone, Debug)]
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
pub fn set_init_matrix(mat: &mut Matrix) {
    for i in 0..3 {
        mat[i] = START_MATRIX[i];
    }
}
pub const START_VECTOR: Vector = Vector {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
/*
 * Calculate the dot product of two vectors sharing a common point.
 * Returns the cosine of the angle between the two vectors.
 */
pub fn vector_dot_product(first: &Vector, second: &Vector) -> f32 {
    return (first.x * second.x) + (first.y * second.y) + (first.z * second.z);
}
pub fn mult_vector(vec: &mut Vector, mat: &Matrix) {
    let mut x: f32;
    let mut y: f32;
    let mut z: f32;
    x = (vec.x * mat[0].x) + (vec.y * mat[0].y) + (vec.z * mat[0].z);

    y = (vec.x * mat[1].x) + (vec.y * mat[1].y) + (vec.z * mat[1].z);

    z = (vec.x * mat[2].x) + (vec.y * mat[2].y) + (vec.z * mat[2].z);

    vec.x = x;
    vec.y = y;
    vec.z = z;
}

/*
 * Convert a vector into a vector of unit (1) length.
 */

pub fn unit_vector(vec: &Vector) -> Vector {
    let mut lx: f32;
    let mut ly: f32;
    let mut lz: f32;
    let mut uni: f32;
    let mut res = START_VECTOR;

    lx = vec.x;
    ly = vec.y;
    lz = vec.z;

    uni = (lx * lx + ly * ly + lz * lz).sqrt();

    res.x = lx / uni;
    res.y = ly / uni;
    res.z = lz / uni;

    return res;
}
pub fn tidy_matrix(mat: &mut Matrix) {
    mat[2] = unit_vector(&mat[2]);

    if ((mat[2].x > -1.0) && (mat[2].x < 1.0)) {
        if ((mat[2].y > -1.0) && (mat[2].y < 1.0)) {
            mat[1].z = -(mat[2].x * mat[1].x + mat[2].y * mat[1].y) / mat[2].z;
        } else {
            mat[1].y = -(mat[2].x * mat[1].x + mat[2].z * mat[1].z) / mat[2].y;
        }
    } else {
        mat[1].x = -(mat[2].y * mat[1].y + mat[2].z * mat[1].z) / mat[2].x;
    }

    mat[1] = unit_vector(&mat[1]);

    /* xyzzy... nothing happens. :-)*/

    mat[0].x = mat[1].y * mat[2].z - mat[1].z * mat[2].y;
    mat[0].y = mat[1].z * mat[2].x - mat[1].x * mat[2].z;
    mat[0].z = mat[1].x * mat[2].y - mat[1].y * mat[2].x;
}
