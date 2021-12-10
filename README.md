# ansipix
A rust library for converting images to ANSI strings to print in a terminal

## Usage
### Add as dependency
Add the following to your `Cargo.toml`
```toml
ansipix = "0.1.0"
```

### Get an ANSI string
```rust
use std::path::PathBuf;
use ansipix::of_image;

let img = of_image(PathBuf::from("path/to/image"), (50, 50), 100);
```
Refer to the doc comments for more information about the parameters.

### Specify a different filter type
`ansipix` uses the `image` crate for opening and resizing the image. The `of_image` function uses `FilterType::Nearest` for resizing. To specify a different one use `of_image_with_filter`:
```rust
use std::path::PathBuf;
use image::imageops::FilterType;
use ansipix::of_image_with_filter;

let img = of_image_with_filter(PathBuf::from("path/to/image"), (32, 32), 255, FilterType::Triangle);
```

### Print Image to the Terminal
```rust
println!("{}", img.unwrap_or_else(|e| {
    println!("Error while opening the file: {}", e);
    std::process::exit(1);
}));
```
