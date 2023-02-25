
use std::num:: {
    cast,
    Float,
    SignedInt,
};
use std::simd::f32x4;

use math::utils::clamp;



pub fn lanczos3_kernel(x: f32) -> f32 {
    lanczos(x, 3.0)
}


pub fn gaussian_kernel(x: f32) -> f32 {
    gaussian(x, 1.0)
}




pub fn triangle_kernel(x: f32) -> f32 {
    if x.abs() < 1.0 {
        1.0 - x
    } else {
        0.0
    }
}


pub fn box_kernel(x: f32) -> f32 {
    if x.abs() <= 0.5 {
        1.0
    } else {
        0.0
    }
}



pub fn catmullrom_kernel(x: f32) -> f32 {
    bc_cubic_spline(x, 0.0, 0.5)
}