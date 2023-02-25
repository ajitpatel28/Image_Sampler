use buffer::{ImageBuffer, Pixel};
use traits::Primitive;
use image::GenericImage;


fn bench_resize(b: &mut test::Bencher) {
    let img = ::open(&Path::new("./examples/fractal.png")).unwrap();
    b.iter(|| {
        test::black_box(resize(&img, 200, 200, ::Nearest ));
    });
    b.bytes = 800*800*3 + 200*200*3;
}


