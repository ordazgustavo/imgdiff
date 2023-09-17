# imgdiff

A command-line and library for comparing two images.

## Features

- Provide visual diff output.

## Installation

Ensure you have Rust and Cargo installed. You can then install the tool with:

```bash
cargo install imgdiff
```

## Usage

```bash
imgdiff image1.png image2.png out.png
```

## Acknowledgments

- This project uses the [image](https://crates.io/crates/image) crate for image processing.

## Roadmap

- Add support thresholds.
- Make overlay color configurable
- Optionally generate output image and report difference instead.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
