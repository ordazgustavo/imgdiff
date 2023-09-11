use std::{
    fmt,
    path::Path,
    time::{Duration, Instant},
};

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1_000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1_000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

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

    let timer = Instant::now();
    let img_c = imgdiff::compare(img_a, img_b);
    println!("Matched in: {}", Elapsed::from(&timer));

    let output_path = Path::new(&output_path);

    img_c.save(output_path).unwrap();
}
