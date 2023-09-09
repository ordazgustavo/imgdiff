use std::env;
use std::path::Path;

use image::{GenericImageView, ImageBuffer, RgbaImage};

fn main() {
    let (file_a, file_b) = if env::args().count() == 3 {
        (env::args().nth(1).unwrap(), env::args().nth(2).unwrap())
    } else {
        panic!("Please enter a file")
    };

    let img_a = image::open(Path::new(&file_a)).unwrap();
    let img_b = image::open(Path::new(&file_b)).unwrap();

    let size_a = img_a.dimensions();
    let size_b = img_b.dimensions();
    let size_c = size_a.max(size_b);

    let mut img_c: RgbaImage =
        ImageBuffer::from_pixel(size_c.0, size_c.1, image::Rgba([255, 255, 255, 255]));

    println!("Are equal: {}", img_a == img_b);

    img_a
        .into_rgba8()
        .enumerate_pixels()
        .zip(img_b.into_rgba8().enumerate_pixels())
        .filter(|((_, _, a), (_, _, b))| a != b)
        .map(|((x, y, _p), _)| (x, y))
        .for_each(|(x, y)| img_c.put_pixel(x, y, image::Rgba([255, 0, 0, 255])));

    img_c.save("c.png").unwrap();
}
