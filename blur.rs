pub fn blur<I: GenericImage + 'static>(image: &I, sigma: f32)
    -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
    where I::Pixel: 'static,
          <I::Pixel as Pixel>::Subpixel: 'static {

    let sigma = if sigma < 0.0 {
        1.0
    } else {
        sigma
    };

    let mut method = Filter {
        kernel: Box::new(|&: x| gaussian(x, sigma)),
        support: 2.0 * sigma
    };
    
    let (width, height) = image.dimensions();

    // Keep width and height the same for horizontal and
    // vertical sampling.
    let tmp = vertical_sample(image, height, &mut method);
    horizontal_sample(&tmp, width, &mut method)
}