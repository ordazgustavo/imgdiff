use std::path::Path;

use image::{DynamicImage, ImageBuffer, ImageResult, Rgba, RgbaImage};

/// Takes two images and computes the difference pixel by pixel.
/// The result is a new image using the `reference` image as
/// the base and a color overlay with the difference from the
/// `current` image.
///
/// Returns `None` if the images are equal
pub fn compare<P>(reference: P, current: P) -> ImageResult<DynamicImage>
where
    P: AsRef<Path>,
{
    let reference = image::open(reference)?;
    let current = image::open(current)?;

    cmp(reference, current)
}

pub(crate) fn cmp(reference: DynamicImage, current: DynamicImage) -> ImageResult<DynamicImage> {
    if reference == current {
        return Ok(reference);
    }

    let (reference, current) = adjust_dymensions(reference, current);

    Ok(diff(reference, current).into())
}

fn adjust_dymensions(reference: DynamicImage, current: DynamicImage) -> (RgbaImage, RgbaImage) {
    let reference = reference.into_rgba8();
    let current = current.into_rgba8();

    let (reference, current) = if reference.dimensions() != current.dimensions() {
        let width = reference.width().max(current.width());
        let height = reference.height().max(current.height());

        let r = resize(&reference, width, height);
        let c = resize(&current, width, height);
        (r, c)
    } else {
        (reference, current)
    };

    (reference, current)
}

fn resize(img: &RgbaImage, x: u32, y: u32) -> RgbaImage {
    let mut resized: RgbaImage = ImageBuffer::new(x, y);
    image::imageops::overlay(&mut resized, img, 0, 0);
    resized
}

pub(crate) const OVERLAY_COLOR: Rgba<u8> = Rgba([255, 0, 0, 55]);

fn diff(reference: RgbaImage, current: RgbaImage) -> RgbaImage {
    let mut c = ImageBuffer::new(reference.width(), reference.height());

    reference
        .enumerate_pixels()
        .zip(current.enumerate_pixels())
        .filter_map(|((xr, yr, r), (xc, yc, c))| {
            (xr == xc && yr == yc && r != c).then_some((xr, yr))
        })
        .for_each(|(x, y)| c.put_pixel(x, y, OVERLAY_COLOR));

    let mut result = reference.clone();

    image::imageops::overlay(&mut result, &c, 0, 0);

    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    type Pix<'a> = &'a [&'a [image::Rgba<u8>]];

    const R: Rgba<u8> = Rgba([255, 0, 0, 255]);
    const G: Rgba<u8> = Rgba([0, 255, 0, 255]);
    const B: Rgba<u8> = Rgba([0, 0, 255, 255]);
    const T: Rgba<u8> = Rgba([0, 0, 0, 0]);
    const O: Rgba<u8> = OVERLAY_COLOR;

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

    fn gen_img(pixels: Pix) -> DynamicImage {
        let (x, y) = (pixels[0].len() as u32, pixels.len() as u32);

        ImageBuffer::from_fn(x, y, |x, y| pixels[y as usize][x as usize]).into()
    }

    #[test]
    fn are_equal() {
        let a = gen_base(3, 1);
        let b = gen_base(3, 1);

        let pixels = cmp(a.clone(), b).unwrap();

        assert_eq!(a, pixels);
    }

    #[test]
    fn first_pixel() {
        let a = gen_base(3, 1);
        let b = gen_img(&[&[G, G, B]]);

        let pixels = cmp(a.clone(), b).unwrap();

        let overlay = gen_img(&[&[OVERLAY_COLOR, T, T]]);
        let mut expected = a;
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, pixels);
    }

    #[test]
    fn last_pixel() {
        let a = gen_base(3, 1);
        let b = gen_img(&[&[R, G, G]]);

        let diff = cmp(a.clone(), b).unwrap();

        let overlay = gen_img(&[&[T, T, OVERLAY_COLOR]]);
        let mut expected = a;
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, diff);
    }

    #[test]
    fn b_x_bigger() {
        let a = gen_base(3, 1);
        let b = gen_img(&[&[R, G, B, B]]);

        let diff = cmp(a, b).unwrap();

        let overlay = gen_img(&[&[T, T, T, OVERLAY_COLOR]]);
        let mut expected = gen_img(&[&[R, G, B, T]]);
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, diff);
    }

    #[test]
    fn b_y_bigger() {
        let a = gen_base(3, 1);
        let b = gen_img(&[&[R, G, B], &[R, G, B]]);

        let diff = cmp(a, b).unwrap();

        let overlay = gen_img(&[&[T, T, T], &[OVERLAY_COLOR, OVERLAY_COLOR, OVERLAY_COLOR]]);
        let mut expected = gen_img(&[&[R, G, B], &[T, T, T]]);
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, diff);
    }

    #[test]
    fn b_xy_bigger() {
        let a = gen_base(3, 1);
        let b = gen_img(&[&[R, G, B, B], &[R, G, B, B]]);

        let pixels = cmp(a, b).unwrap();

        let overlay = gen_img(&[
            &[T, T, T, OVERLAY_COLOR],
            &[OVERLAY_COLOR, OVERLAY_COLOR, OVERLAY_COLOR, OVERLAY_COLOR],
        ]);
        let mut expected = gen_img(&[&[R, G, B, T], &[T, T, T, T]]);
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, pixels);
    }

    #[test]
    fn a_x_bigger() {
        let a = gen_base(4, 1);
        let b = gen_img(&[&[R, G, B]]);

        let diff = cmp(a.clone(), b).unwrap();

        let overlay = gen_img(&[&[T, T, T, OVERLAY_COLOR]]);
        let mut expected = a;
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, diff);
    }

    #[test]
    fn a_x_bigger_and_diff_content() {
        let a = gen_base(4, 1);
        let b = gen_img(&[&[R, B, B]]);

        let diff = cmp(a.clone(), b).unwrap();

        let overlay = gen_img(&[&[T, OVERLAY_COLOR, T, OVERLAY_COLOR]]);
        let mut expected = a;
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, diff);
    }

    #[test]
    fn a_y_bigger() {
        let a = gen_base(3, 2);
        let b = gen_img(&[&[R, G, B]]);

        let diff = cmp(a.clone(), b).unwrap();

        let overlay = gen_img(&[&[R, G, B], &[OVERLAY_COLOR, OVERLAY_COLOR, OVERLAY_COLOR]]);
        let mut expected = a;
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, diff);
    }

    #[test]
    fn a_y_bigger_and_diff_content() {
        let a = gen_base(3, 2);
        let b = gen_img(&[&[R, B, B]]);

        let diff = cmp(a.clone(), b).unwrap();

        let overlay = gen_img(&[
            &[T, OVERLAY_COLOR, T],
            &[OVERLAY_COLOR, OVERLAY_COLOR, OVERLAY_COLOR],
        ]);
        let mut expected = a;
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, diff);
    }

    #[test]
    fn a_xy_bigger() {
        let a = gen_base(4, 2);
        let b = gen_img(&[&[R, G, B]]);

        let diff = cmp(a.clone(), b).unwrap();

        let overlay = gen_img(&[
            &[T, T, T, OVERLAY_COLOR],
            &[OVERLAY_COLOR, OVERLAY_COLOR, OVERLAY_COLOR, OVERLAY_COLOR],
        ]);
        let mut expected = a;
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, diff);
    }

    #[test]
    fn different_size_and_content() {
        let a = gen_base(6, 6);
        let b = gen_img(&[&[B, G], &[R, G]]);

        let diff = cmp(a.clone(), b).unwrap();

        let overlay = gen_img(&[
            &[O, T, O, O, O, O],
            &[T, T, O, O, O, O],
            &[O, O, O, O, O, O],
            &[O, O, O, O, O, O],
            &[O, O, O, O, O, O],
            &[O, O, O, O, O, O],
        ]);
        let mut expected = a;
        image::imageops::overlay(&mut expected, &overlay, 0, 0);

        assert_eq!(expected, diff);
    }
}
