fn vertical_sample<I, P, S>(image: &I, new_height: u32,
    filter: &mut Filter)
-> ImageBuffer<P, Vec<S>>
where I: GenericImage<Pixel=P> + 'static,
P: Pixel<Subpixel=S> + 'static,
S: Primitive + 'static {

let (width, height) = image.dimensions();
let mut out = ImageBuffer::new(width, new_height);


for x in (0..width) {
let max: S = Primitive::max_value();
let max: f32 = cast(max).unwrap();

let ratio = height as f32 / new_height as f32;

// Scale the filter when downsampling.
let filter_scale = if ratio > 1.0 {
ratio
} else {
1.0
};

let filter_radius = (filter.support * filter_scale).ceil();

for outy in (0..new_height) {
let inputy = (outy as f32 + 0.5) * ratio;

let left  = (inputy - filter_radius).ceil() as i64;
let left  = clamp(left, 0, height as i64 - 1) as u32;

let right = (inputy + filter_radius).floor() as i64;
let right = clamp(right, 0, height as i64 - 1) as u32;

let mut sum = f32x4(0., 0., 0., 0.);

let mut t = f32x4(0., 0., 0., 0.);

for i in (left..right + 1) {
let w = (filter.kernel)((i as f32 - inputy) / filter_scale);
let w = f32x4(w, w, w, w);
sum += w;

let y0  = clamp(i, 0, height - 1);
let p = image.get_pixel(x, y0);

let (k1, k2, k3, k4) = p.channels4();
let vec = f32x4(
cast(k1).unwrap(),
cast(k2).unwrap(),
cast(k3).unwrap(),
cast(k4).unwrap()
);

t += vec * w;
}

let f32x4(t1, t2, t3, t4) = t / sum;
let t = Pixel::from_channels(
cast(clamp(t1, 0.0, max)).unwrap(),
cast(clamp(t2, 0.0, max)).unwrap(),
cast(clamp(t3, 0.0, max)).unwrap(),
cast(clamp(t4, 0.0, max)).unwrap()
);

out.put_pixel(x, outy, t);
}
}

out
}
