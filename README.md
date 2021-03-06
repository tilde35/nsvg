# nsvg [![Build Status](https://travis-ci.org/nickbrowne/nsvg.svg?branch=master)](https://travis-ci.org/nickbrowne/nsvg) [![Build status](https://ci.appveyor.com/api/projects/status/nr4jsaibmh5i3fxw/branch/master?svg=true)](https://ci.appveyor.com/project/nickbrowne/nsvg/branch/master) [![Crates.io](https://img.shields.io/crates/v/nsvg.svg)](https://crates.io/crates/nsvg)

A friendly Rust wrapper around the excellent NanoSVG C library. Offering simple SVG parsing and rasterizing.

Does not provide all the functionality of NanoSVG yet. Just the bare minimum to create scaled rasters of SVGs.

Like NanoSVG, the rasteriser only renders flat filled shapes. It is not particularly fast or accurate, but it is a simple way to bake vector graphics into textures.

https://github.com/memononen/nanosvg

https://github.com/rezrov/libnanosvg

## Usage

Include `nsvg` in your `Cargo.toml` dependencies.

```toml
[dependencies]
nsvg = "0.5.0"
```

To include `nsvg` without the `image` dependency, add it to your `Cargo.toml` without default dependencies.

```toml
[dependencies.nsvg]
version = "0.5.0"
default-features = false
```

Now you can parse and rasterize SVGs. Use the scale argument to produce larger or smaller rasterised images. The aspect ratio will remain the same.

This can be useful to create rasters to suit high resolution displays.

```rust
extern crate nsvg;

use std::path::Path;

fn main() {
  let path = Path::new("examples/spiral.svg");

  // Load and parse the svg
  let svg = nsvg::parse_file(path, nsvg::Units::Pixel, 96.0).unwrap();

  // Create a scaled raster
  let scale = 2.0;
  let image = svg.rasterize(scale);
}

```

## Developing

By default nsvg will use prebuilt FFI bindings, but you can regenerate them manually by running:

```
cargo build --features bindgen
```

Which will also copy the bindings into the `src` directory. If the bindings need to be updated (when upgrading NanoSVG, for example) they should be checked in to version control.

Make sure you have `libclang-dev` available so bindgen works.

`cargo test` to run tests.

There is one provided example, which you can run with the following command:
```
cargo run --example svg_to_png
```

This will convert the `spiral.svg` vector into a PNG. The PNG will be written to the working directory as `spiral_output.png`.

## License

NanoSVG is licensed under [zlib license](lib/LICENSE.txt)

Anything else is MIT
