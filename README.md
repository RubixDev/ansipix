# ansipix

A Rust library for converting images to ANSI strings to print in a terminal

## Usage

### Get an ANSI string

```rust
let img = ansipix::of_image(&image::open("example.png").unwrap(), (50, 50), 100, false);
println!("{}", img);
```

Refer to the [docs](https://docs.rs/ansipix/latest/ansipix/) for more
information.

### Specify a different filter type

`ansipix` uses the [`image`](https://docs.rs/image/latest/image/) crate for
reading and resizing the image. The [`of_image`] function uses
[`FilterType::Nearest`] for resizing. You can specify a different one with the
[`of_image_with_filter`] function.

```rust
use image::imageops::FilterType;

let img = ansipix::of_image_with_filter(&image::open("example.png").unwrap(), (32, 32), 255, false, FilterType::Triangle);
println!("{}", img);
```
