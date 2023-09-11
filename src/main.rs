use std::path::Path;

fn main() {
    let args = std::env::args();
    let mut args = args.skip(1);

    let (file_a, file_b, output_path) = (
        args.next().expect("Expected argument"),
        args.next().expect("Expected argument"),
        args.next().expect("Expected argument"),
    );

    let img_a = image::open(Path::new(&file_a)).unwrap();
    let img_b = image::open(Path::new(&file_b)).unwrap();

    let img_c = imgdiff::diff(img_a.into_rgb8(), img_b.into_rgb8());

    let output_path = Path::new(&output_path);

    img_c.save(output_path).unwrap();
}
