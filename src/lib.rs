use image::{ImageBuffer, Rgb, RgbImage};

pub fn diff(a: RgbImage, b: RgbImage) -> RgbImage {
    let size_a = a.dimensions();
    let size_b = b.dimensions();

    let mut c: RgbImage = if size_a != size_b {
        let (x, y) = size_a.max(size_b);

        ImageBuffer::from_fn(x, y, |x, y| {
            // Fill excess size with red pixels
            if x >= size_a.0 || y >= size_a.1 || x >= size_b.0 || y >= size_b.1 {
                Rgb([255, 0, 0])
            } else {
                Rgb([255, 255, 255])
            }
        })
    } else {
        ImageBuffer::from_pixel(size_a.0, size_a.1, Rgb([255, 255, 255]))
    };

    a.enumerate_pixels()
        .zip(b.enumerate_pixels())
        .filter_map(|((xa, ya, a), (xb, yb, b))| {
            (xa == xb && ya == yb && a != b).then_some((xa, ya))
        })
        .for_each(|(x, y)| c.put_pixel(x, y, Rgb([255, 0, 0])));

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    type Pix = &'static [&'static [image::Rgb<u8>]];

    const W: Rgb<u8> = Rgb([255, 255, 255]);
    const R: Rgb<u8> = Rgb([255, 0, 0]);
    const G: Rgb<u8> = Rgb([0, 255, 0]);
    const B: Rgb<u8> = Rgb([0, 0, 255]);

    const BASE_PATTERN: [Rgb<u8>; 3] = [R, G, B];

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
    fn gen_base(x: u32, y: u32) -> RgbImage {
        let mut pixels = BASE_PATTERN.iter().cycle();

        ImageBuffer::from_fn(x, y, |_, _| *pixels.next().unwrap())
    }

    fn gen_img(pixels: Pix) -> RgbImage {
        let (x, y) = (pixels[0].len() as u32, pixels.len() as u32);

        ImageBuffer::from_fn(x, y, |x, y| pixels[y as usize][x as usize])
    }

    #[test]
    fn are_equal() {
        let a = gen_base(3, 1);
        let b = gen_base(3, 1);

        let pixels = diff(a, b);

        let expected: Pix = &[&[W, W, W]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn just_one_pixel() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[G, G, B]];

        let pixels = diff(a, gen_img(b));

        let expected: Pix = &[&[R, W, W]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn last_two_pixels() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[R, G, G]];

        let pixels = diff(a, gen_img(b));

        let expected: Pix = &[&[W, W, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn b_x_bigger() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[R, G, B, B]];

        let pixels = diff(a, gen_img(b));

        let expected: Pix = &[&[W, W, W, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn b_y_bigger() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[R, G, B], &[R, G, B]];

        let pixels = diff(a, gen_img(b));

        let expected: Pix = &[&[W, W, W], &[R, R, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn b_xy_bigger() {
        let a = gen_base(3, 1);
        let b: Pix = &[&[R, G, B, B], &[R, G, B, B]];

        let pixels = diff(a, gen_img(b));

        let expected: Pix = &[&[W, W, W, R], &[R, R, R, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_x_bigger() {
        let a = gen_base(4, 1);
        let b: Pix = &[&[R, G, B]];

        let pixels = diff(a, gen_img(b));

        let expected: Pix = &[&[W, W, W, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_y_bigger() {
        let a = gen_base(3, 2);
        let b: Pix = &[&[R, G, B]];

        let pixels = diff(a, gen_img(b));

        let expected: Pix = &[&[W, W, W], &[R, R, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_xy_bigger() {
        let a = gen_base(4, 2);
        let b: Pix = &[&[R, G, B]];

        let pixels = diff(a, gen_img(b));

        let expected: Pix = &[&[W, W, W, R], &[R, R, R, R]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn size_and_different() {
        let a = gen_base(3, 3);
        let b: Pix = &[&[B, G], &[R, G]];

        let pixels = diff(a, gen_img(b));

        let expected: Pix = &[&[R, W, R], &[W, W, R], &[R, R, R]];

        assert_eq!(gen_img(expected).into_vec(), pixels.into_vec());
    }
}
