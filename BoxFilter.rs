pub fn filter3x3<I, P, S>(image: &I, kernel: &[f32])
    -> ImageBuffer<P, Vec<S>>
    where I: GenericImage<Pixel=P> + 'static,
          P: Pixel<Subpixel=S> + 'static,
          S: Primitive + 'static {

    // The kernel's input positions relative to the current pixel.
    let taps: &[(isize, isize)] = &[
        (-1, -1), ( 0, -1), ( 1, -1),
        (-1,  0), ( 0,  0), ( 1,  0),
        (-1,  1), ( 0,  1), ( 1,  1),
      ];

    let (width, height) = image.dimensions();

    let mut out = ImageBuffer::new(width, height);

    let max: S = Primitive::max_value();
    let max: f32 = cast(max).unwrap();

    let sum = match kernel.iter().fold(0.0, |&: a, f| a + *f) {
        0.0 => 1.0,
        sum => sum
    };
    let sum = f32x4(sum, sum, sum, sum);

    for y in (1..height - 1) {
        for x in (1..width - 1) {
            let mut t = f32x4(0., 0., 0., 0.);


            // TODO: There is no need to recalculate the kernel for each pixel.
            // Only a subtract and addition is needed for pixels after the first
            // in each row.
            for (&k, &(a, b)) in kernel.iter().zip(taps.iter()) {
                let k = f32x4(k, k, k, k);
                let x0 = x as isize + a;
                let y0 = y as isize + b;

                let p = image.get_pixel(x0 as u32, y0 as u32);

                let (k1, k2, k3, k4) = p.channels4();

                let vec = f32x4(
                    cast(k1).unwrap(),
                    cast(k2).unwrap(),
                    cast(k3).unwrap(),
                    cast(k4).unwrap()
                );

                t += vec * k;
            }

            let f32x4(t1, t2, t3, t4) = t / sum;

            let t = Pixel::from_channels(
                cast(clamp(t1, 0.0, max)).unwrap(),
                cast(clamp(t2, 0.0, max)).unwrap(),
                cast(clamp(t3, 0.0, max)).unwrap(),
                cast(clamp(t4, 0.0, max)).unwrap()
            );

            out.put_pixel(x, y, t);
        }
    }

    out
}