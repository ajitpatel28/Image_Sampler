use std::f32;

use std::num:: {
    cast,
    Float,
    SignedInt,
};
use std::simd::f32x4;

use math::utils::clamp;



pub fn gaussian(x: f32, r: f32) -> f32 {
    ((2.0 * f32::consts::PI).sqrt() * r).recip() *
    (-x.powi(2) / (2.0 * r.powi(2))).exp()
}




