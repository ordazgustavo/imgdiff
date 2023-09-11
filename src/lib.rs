use image::{ImageBuffer, Rgb, RgbImage};

pub fn diff(a: RgbImage, b: RgbImage) -> RgbImage {
    let size_a = a.dimensions();
    let size_b = b.dimensions();

    let (x, y) = size_a.max(size_b);

    let mut c: RgbImage = ImageBuffer::from_fn(x, y, |x, y| {
        // Fill excess size with red pixels
        if x >= size_a.0 || y >= size_a.1 || x >= size_b.0 || y >= size_b.1 {
            Rgb([255, 0, 0])
        } else {
            Rgb([255, 255, 255])
        }
    });

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

    const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
    const RED: Rgb<u8> = Rgb([255, 0, 0]);
    const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
    const BLUE: Rgb<u8> = Rgb([0, 0, 255]);

    fn gen_img(pixels: Pix) -> RgbImage {
        let (x, y) = (pixels[0].len() as u32, pixels.len() as u32);

        ImageBuffer::from_fn(x, y, |x, y| pixels[y as usize][x as usize])
    }

    #[test]
    fn are_equal() {
        let a: Pix = &[&[RED, GREEN, BLUE]];
        let b: Pix = &[&[RED, GREEN, BLUE]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[WHITE, WHITE, WHITE]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn just_one_pixel() {
        let a: Pix = &[&[RED, GREEN, BLUE]];
        let b: Pix = &[&[GREEN, GREEN, BLUE]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[RED, WHITE, WHITE]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn last_two_pixels() {
        let a: Pix = &[&[RED, GREEN, BLUE]];
        let b: Pix = &[&[RED, GREEN, GREEN]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[WHITE, WHITE, RED]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn b_x_bigger() {
        let a: Pix = &[&[RED, GREEN, BLUE]];
        let b: Pix = &[&[RED, GREEN, BLUE, BLUE]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[WHITE, WHITE, WHITE, RED]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn b_y_bigger() {
        let a: Pix = &[&[RED, GREEN, BLUE]];
        let b: Pix = &[&[RED, GREEN, BLUE], &[RED, GREEN, BLUE]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[WHITE, WHITE, WHITE], &[RED, RED, RED]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn b_xy_bigger() {
        let a: Pix = &[&[RED, GREEN, BLUE]];
        let b: Pix = &[&[RED, GREEN, BLUE, BLUE], &[RED, GREEN, BLUE, BLUE]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[WHITE, WHITE, WHITE, RED], &[RED, RED, RED, RED]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_x_bigger() {
        let a: Pix = &[&[RED, GREEN, BLUE, BLUE]];
        let b: Pix = &[&[RED, GREEN, BLUE]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[WHITE, WHITE, WHITE, RED]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_y_bigger() {
        let a: Pix = &[&[RED, GREEN, BLUE], &[RED, GREEN, BLUE]];
        let b: Pix = &[&[RED, GREEN, BLUE]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[WHITE, WHITE, WHITE], &[RED, RED, RED]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn a_xy_bigger() {
        let a: Pix = &[&[RED, GREEN, BLUE, BLUE], &[RED, GREEN, BLUE, BLUE]];
        let b: Pix = &[&[RED, GREEN, BLUE]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[WHITE, WHITE, WHITE, RED], &[RED, RED, RED, RED]];

        assert_eq!(gen_img(expected), pixels);
    }

    #[test]
    fn size_and_different() {
        let a: Pix = &[
            &[RED, GREEN, BLUE],
            &[RED, GREEN, BLUE],
            &[RED, GREEN, BLUE],
        ];
        let b: Pix = &[&[BLUE, GREEN], &[RED, GREEN]];

        let pixels = diff(gen_img(a), gen_img(b));

        // expected
        let expected: Pix = &[&[RED, WHITE, RED], &[WHITE, WHITE, RED], &[RED, RED, RED]];

        assert_eq!(gen_img(expected).into_vec(), pixels.into_vec());
    }
}
