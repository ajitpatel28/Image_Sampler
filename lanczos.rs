use std::f32;

use std::num:: {
    cast,
    Float,
    SignedInt,
};


fn lanczos(x: f32, t: f32) -> f32 {
    if x.abs() < t {
        sinc(x) * sinc(x / t)
    } else {
        0.0
    }
}



