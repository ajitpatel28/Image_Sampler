pub fn unsharpen<I, P, S>(image: &I, sigma: f32, threshold: i32)
    -> ImageBuffer<P, Vec<S>>
    where I: GenericImage<Pixel=P> + 'static,
          P: Pixel<Subpixel=S> + 'static,
          S: Primitive + 'static {

    let mut tmp = blur(image, sigma);

    let max: S = Primitive::max_value();
    let max: i32 = cast(max).unwrap();
    let (width, height) = image.dimensions();

    for y in (0..height) {
        for x in (0..width) {
            let a = image.get_pixel(x, y);
            let b = tmp.get_pixel_mut(x, y);

            let p = a.map2(b, |&: c, d| {
                let ic: i32 = cast(c).unwrap();
                let id: i32 = cast(d).unwrap();

                let diff = (ic - id).abs();

                if diff > threshold {
                let e = clamp(ic + diff, 0, max);

                    cast(e).unwrap()
                } else {
                    c
                }
            });

            *b = p;
        }
    }

    tmp
}

#[cfg(test)]
mod tests {
    use test;
    use buffer::{ImageBuffer, RgbImage};
    use super::{resize, FilterType};

    #[bench]
    fn bench_resize(b: &mut test::Bencher) {
        let img = ::open(&Path::new("./examples/fractal.png")).unwrap();
        b.iter(|| {
            test::black_box(resize(&img, 200, 200, ::Nearest ));
        });
        b.bytes = 800*800*3 + 200*200*3;
    }

    #[test]
    fn test_issue_186() {
        let img: RgbImage = ImageBuffer::new(100, 100);
        let _ = resize(&img, 50, 50, FilterType::Lanczos3);
    }

}