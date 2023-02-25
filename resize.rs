pub fn resize<I: GenericImage + 'static>(image: &I, nwidth: u32, nheight: u32,
    filter: FilterType)
-> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
where I::Pixel: 'static,
<I::Pixel as Pixel>::Subpixel: 'static {

let mut method = match filter {
FilterType::Nearest    =>   Filter {
kernel: Box::new(|&: x| box_kernel(x)),
support: 0.5
},
FilterType::Triangle   => Filter {
kernel: Box::new(|&: x| triangle_kernel(x)),
support: 1.0
},
FilterType::CatmullRom => Filter {
kernel: Box::new(|&: x| catmullrom_kernel(x)),
support: 2.0
},
FilterType::Gaussian   => Filter {
kernel: Box::new(|&: x| gaussian_kernel(x)),
support: 3.0
},
FilterType::Lanczos3   => Filter {
kernel: Box::new(|&: x| lanczos3_kernel(x)),
support: 3.0
},
};

let tmp = vertical_sample(image, nheight, &mut method);
horizontal_sample(&tmp, nwidth, &mut method)
}


#[cfg(test)]
mod tests {
    use test;
    use buffer::{ImageBuffer, RgbImage};
    use super::{resize, FilterType};

    #[test]
    fn test_issue_186() {
        let img: RgbImage = ImageBuffer::new(100, 100);
        let _ = resize(&img, 50, 50, FilterType::Lanczos3);
    }

}
