extern crate nsvg;
extern crate image;

use std::env;

fn main() {
  // Load the SVG data
  let svg = nsvg::parse_file("examples/spiral.svg", nsvg::Units::Pixel, 96.0).unwrap();

  // Rasterize the loaded SVG and return an RgbaImage
  let image = svg.rasterize(2.0).unwrap();

  let save_path = env::current_dir().unwrap().join("spiral_output.png");
  let (width, height) = image.dimensions();

  // Write the image to disk as a PNG
  image::save_buffer(
    save_path,
    &image.into_raw(),
    width,
    height,
    image::ColorType::RGBA(8),
  ).expect("Failed to save png.");
}
