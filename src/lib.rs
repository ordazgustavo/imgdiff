use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

pub fn compare(a: DynamicImage, b: DynamicImage) -> RgbaImage {
    if a == b {
        return a.into_rgba8();
    }

    let a = a.into_rgba8();
    let b = b.into_rgba8();

    let (a, b) = if a.dimensions() != b.dimensions() {
        let width = a.width().max(b.width());
        let height = a.height().max(b.height());

        let a = resize(&a, width, height);
        let b = resize(&b, width, height);
        (a, b)
    } else {
        (a, b)
    };

    diff(a, b)
}

fn resize(img: &RgbaImage, x: u32, y: u32) -> RgbaImage {
    let mut resized: RgbaImage = ImageBuffer::new(x, y);
    image::imageops::overlay(&mut resized, img, 0, 0);
    resized
}

fn diff(a: RgbaImage, b: RgbaImage) -> RgbaImage {
    let mut c = ImageBuffer::new(a.width(), a.height());

    a.enumerate_pixels()
        .zip(b.enumerate_pixels())
        .filter_map(|((xa, ya, a), (xb, yb, b))| {
            (xa == xb && ya == yb && a != b).then_some((xa, ya))
        })
        .for_each(|(x, y)| c.put_pixel(x, y, Rgba([255, 0, 0, 55])));

    let mut a = a.clone();

    image::imageops::overlay(&mut a, &c, 0, 0);

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    type Pix<'a> = &'a [&'a [image::Rgba<u8>]];

    const R: Rgba<u8> = Rgba([255, 0, 0, 255]);
    const G: Rgba<u8> = Rgba([0, 255, 0, 255]);
    const B: Rgba<u8> = Rgba([0, 0, 255, 255]);

    const BASE_PATTERN: [Rgba<u8>; 3] = [R, G, B];

    /// Give a size, generate an image by cycling through an `[R, G, B]` array
    ///
    /// # Example
    ///
    /// `gen_base(3, 3)`
    ///
    /// ```
    /// R G B
    /// R G B
    /// R G B
    /// ```
    fn gen_base(x: u32, y: u32) -> DynamicImage {
        let mut pixels = BASE_PATTERN.iter().cycle();

        ImageBuffer::from_fn(x, y, |_, _| *pixels.next().unwrap()).into()
    }

    fn gen_img(pixels: Pix) -> RgbaImage {
        let (x, y) = (pixels[0].len() as u32, pixels.len() as u32);

        ImageBuffer::from_fn(x, y, |x, y| pixels[y as usize][x as usize]).into()
    }

    #[test]
    fn are_equal() {
        let a = gen_base(3, 1);
        let b = gen_base(3, 1);

        let pixels = compare(a, b);

        let expected: Pix = &[&[R, G, B]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn just_one_pixel() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[G, G, B]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, G, B]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn last_two_pixels() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[R, G, G]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, G, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn b_x_bigger() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[R, G, B, B]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, G, B, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn b_y_bigger() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[R, G, B], &[R, G, B]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, G, B], &[R, R, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn b_xy_bigger() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[R, G, B, B], &[R, G, B, B]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, G, B, R], &[R, R, R, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_x_bigger() {
        let a = gen_base(4, 1);
        let b: Pix = &[&[R, G, B]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, G, B, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_x_bigger_and_diff_content() {
        let a = gen_base(4, 1);
        let b: Pix = &[&[R, B, B]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, R, B, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_y_bigger() {
        let a = gen_base(3, 2);
        let b: Pix = &[&[R, G, B]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, G, B], &[R, R, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_y_bigger_and_diff_content() {
        let a = gen_base(3, 2);
        let b: Pix = &[&[R, B, B]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, R, B], &[R, R, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_xy_bigger() {
        let a = gen_base(4, 2);
        let b: Pix = &[&[R, G, B]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[&[R, G, B, R], &[R, R, R, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn different_size_and_content() {
        let a = gen_base(6, 6);
        let b: Pix = &[&[B, G], &[R, G]];

        let pixels = compare(a, gen_img(b).into());

        let expected: Pix = &[
            &[R, G, R, R, R, R],
            &[R, G, R, R, R, R],
            &[R, R, R, R, R, R],
            &[R, R, R, R, R, R],
            &[R, R, R, R, R, R],
            &[R, R, R, R, R, R],
        ];

        assert_eq!(gen_img(expected).into_vec(), pixels.into_vec());
    }
}
