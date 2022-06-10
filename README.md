# ansipix
A rust library for converting images to ANSI strings to print in a terminal

## Usage
### Add as dependency
Add the following to your `Cargo.toml`
```toml
ansipix = "1.0.0"
```

### Get an ANSI string
```rust
use std::path::PathBuf;

let img = ansipix::of_image_file(PathBuf::from("example.png"), (50, 50), 100, false);

match img {
    Ok(img) => println!("{}", img),
    Err(e) => eprintln!("{}", e),
}
```
Refer to the [docs](https://docs.rs/ansipix/latest/ansipix/) for more information.

### Specify a different filter type
`ansipix` uses the `image` crate for opening and resizing the image. The `of_image_file` function uses `FilterType::Nearest` for resizing. You can specify a different one with the `of_image_file_with_filter` function.
```rust
use std::path::PathBuf;
use ansipix::FilterType;

let img = ansipix::of_image_file_with_filter(PathBuf::from("example.png"), (32, 32), 255, false, FilterType::Triangle);
match img {
    Ok(img) => println!("{}", img),
    Err(e) => eprintln!("{}", e),
}
```
