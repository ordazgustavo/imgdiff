use std::{
    fmt,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use argh::FromArgs;

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

#[derive(FromArgs)]
/// Compare two images and place the difference in the <out> path.
/// If the images are equal, no image will be saved.
struct Args {
    #[argh(positional)]
    /// path to the reference image
    reference: PathBuf,
    #[argh(positional)]
    /// path to the current image
    current: PathBuf,
    #[argh(positional)]
    /// path to put the resulting image
    out: PathBuf,
}

fn main() {
    let Args {
        reference,
        current,
        out,
    } = argh::from_env::<Args>();

    let timer = Instant::now();

    let result = imgdiff::compare(reference, current);

    let elapsed = Elapsed::from(&timer);

    match result {
        Err(err) => eprintln!("{err}"),
        Ok(result) => {
            let output_path = Path::new(&out);

            if let Err(err) = result.save(output_path) {
                eprintln!("{err}");
            } else {
                println!("Matched in: {elapsed}");
            }
        }
    }
}
