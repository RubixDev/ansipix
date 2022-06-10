#![doc = include_str!("../README.md")]

use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

pub use image::{imageops::FilterType, ImageFormat, ImageResult};

const TOP_HALF: &str = "\u{2580}";
const BOTTOM_HALF: &str = "\u{2584}";

/// Open an image and convert it to a `String`, with [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) for color, while specifying a filter for resizing
///
/// ## Params
/// - `image: image::DynamicImage` - The image
/// - `size: (usize, usize)` - The maximum size of the resized image in `(width, height)` notation
/// - `alpha_threshold: u8` - Minimum alpha value of a pixel for it to be shown. `0` for no transparent background
/// - `raw: bool` - Whether to print the escape sequences literal
/// - `resize_filter: image::imageops::FilterType` - The filter to be used for resizing the image
///
/// ## Returns
/// A `String` with the image
fn of_image_with_filter(
    image: DynamicImage,
    size: (usize, usize),
    alpha_threshold: u8,
    raw: bool,
    resize_filter: FilterType,
) -> String {
    let esc = if raw { "\\x1b" } else { "\x1b" };

    let mut pixels: Vec<Vec<[u8; 4]>> = vec![];
    for (x, y, pix) in image
        .resize(size.0 as u32, size.1 as u32, resize_filter)
        .pixels()
    {
        if x == 0 {
            pixels.push(vec![]);
        }
        pixels[y as usize].push(pix.0);
    }

    let mut out: String = String::new();
    for line in (0..pixels.len()).filter(|index| index % 2 == 0) {
        for char in 0..pixels[line].len() {
            let top_pix: [u8; 4] = pixels[line][char];
            let bot_pix: [u8; 4] = if line + 1 >= pixels.len() {
                [0; 4]
            } else {
                pixels[line + 1][char]
            };
            let top_invis: bool = top_pix[3] < alpha_threshold;
            let bot_invis: bool = bot_pix[3] < alpha_threshold;

            if top_invis && bot_invis {
                out += " ";
            } else if top_invis && !bot_invis {
                out += format!(
                    "{}[38;2;{};{};{}m{}{}[0m",
                    esc, bot_pix[0], bot_pix[1], bot_pix[2], BOTTOM_HALF, esc
                )
                .as_str();
            } else if !top_invis && bot_invis {
                out += format!(
                    "{}[38;2;{};{};{}m{}{}[0m",
                    esc, top_pix[0], top_pix[1], top_pix[2], TOP_HALF, esc
                )
                .as_str();
            } else {
                out += format!(
                    "{}[38;2;{};{};{};48;2;{};{};{}m{}{}[0m",
                    esc,
                    bot_pix[0],
                    bot_pix[1],
                    bot_pix[2],
                    top_pix[0],
                    top_pix[1],
                    top_pix[2],
                    BOTTOM_HALF,
                    esc
                )
                .as_str();
            }
        }
        out += "\n";
    }
    return out;
}

/// Open an image and convert it to a `String`, with [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) for color
///
/// ## Params
/// - `file: std::path::PathBuf` - The path to the image
/// - `size: (usize, usize)` - The maximum size of the resized image in `(width, height)` notation
/// - `alpha_threshold: u8` - Minimum alpha value of a pixel for it to be shown. `0` for no transparent background
/// - `raw: bool` - Whether to print the escape sequences literal
/// - `resize_filter: image::imageops::FilterType` - The filter to be used for resizing the image
///
/// ## Returns
/// A `String` with the image when the specified `file` could be opened as an image, otherwise an `image::error::ImageError`
pub fn of_image_file_with_filter(
    file: PathBuf,
    size: (usize, usize),
    alpha_threshold: u8,
    raw: bool,
    resize_filter: FilterType,
) -> ImageResult<String> {
    Ok(of_image_with_filter(
        image::open(file)?,
        size,
        alpha_threshold,
        raw,
        resize_filter,
    ))
}

/// Open an image and convert it to a `String`, with [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) for color
///
/// ## Params
/// - `file: std::path::PathBuf` - The path to the image
/// - `size: (usize, usize)` - The maximum size of the resized image in `(width, height)` notation
/// - `alpha_threshold: u8` - Minimum alpha value of a pixel for it to be shown. `0` for no transparent background
/// - `raw: bool` - Whether to print the escape sequences literal
///
/// ## Returns
/// A `String` with the image when the specified `file` could be opened as an image, otherwise an `image::error::ImageError`
pub fn of_image_file(
    file: PathBuf,
    size: (usize, usize),
    alpha_threshold: u8,
    raw: bool,
) -> ImageResult<String> {
    of_image_file_with_filter(file, size, alpha_threshold, raw, FilterType::Nearest)
}

/// Open an image and convert it to a `String`, with [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) for color
///
/// ## Params
/// - `buffer: &[u8]` - The bytes of the image
/// - `size: (usize, usize)` - The maximum size of the resized image in `(width, height)` notation
/// - `alpha_threshold: u8` - Minimum alpha value of a pixel for it to be shown. `0` for no transparent background
/// - `raw: bool` - Whether to print the escape sequences literal
/// - `resize_filter: image::imageops::FilterType` - The filter to be used for resizing the image
///
/// ## Returns
/// A `String` with the image when the specified `buffer` could be read as an image and the format could be detected, otherwise an `image::error::ImageError`
pub fn of_image_bytes_with_filter(
    buffer: &[u8],
    size: (usize, usize),
    alpha_threshold: u8,
    raw: bool,
    resize_filter: FilterType,
) -> ImageResult<String> {
    Ok(of_image_with_filter(
        image::load_from_memory(buffer)?,
        size,
        alpha_threshold,
        raw,
        resize_filter,
    ))
}

/// Open an image and convert it to a `String`, with [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) for color
///
/// ## Params
/// - `buffer: &[u8]` - The bytes of the image
/// - `size: (usize, usize)` - The maximum size of the resized image in `(width, height)` notation
/// - `alpha_threshold: u8` - Minimum alpha value of a pixel for it to be shown. `0` for no transparent background
/// - `raw: bool` - Whether to print the escape sequences literal
///
/// ## Returns
/// A `String` with the image when the specified `buffer` could be read as an image and the format could be detected, otherwise an `image::error::ImageError`
pub fn of_image_bytes(
    buffer: &[u8],
    size: (usize, usize),
    alpha_threshold: u8,
    raw: bool,
) -> ImageResult<String> {
    of_image_bytes_with_filter(buffer, size, alpha_threshold, raw, FilterType::Nearest)
}

/// Open an image and convert it to a `String`, with [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) for color
///
/// ## Params
/// - `buffer: &[u8]` - The bytes of the image
/// - `size: (usize, usize)` - The maximum size of the resized image in `(width, height)` notation
/// - `alpha_threshold: u8` - Minimum alpha value of a pixel for it to be shown. `0` for no transparent background
/// - `raw: bool` - Whether to print the escape sequences literal
/// - `resize_filter: image::imageops::FilterType` - The filter to be used for resizing the image
/// - `format: image::ImageFormat` - The format of the given image
///
/// ## Returns
/// A `String` with the image when the specified `buffer` could be read as an image, otherwise an `image::error::ImageError`
pub fn of_image_bytes_with_filter_and_format(
    buffer: &[u8],
    size: (usize, usize),
    alpha_threshold: u8,
    raw: bool,
    resize_filter: FilterType,
    format: ImageFormat,
) -> ImageResult<String> {
    Ok(of_image_with_filter(
        image::load_from_memory_with_format(buffer, format)?,
        size,
        alpha_threshold,
        raw,
        resize_filter,
    ))
}

/// Open an image and convert it to a `String`, with [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) for color
///
/// ## Params
/// - `buffer: &[u8]` - The bytes of the image
/// - `size: (usize, usize)` - The maximum size of the resized image in `(width, height)` notation
/// - `alpha_threshold: u8` - Minimum alpha value of a pixel for it to be shown. `0` for no transparent background
/// - `raw: bool` - Whether to print the escape sequences literal
/// - `format: image::ImageFormat` - The format of the given image
///
/// ## Returns
/// A `String` with the image when the specified `buffer` could be read as an image, otherwise an `image::error::ImageError`
pub fn of_image_bytes_with_format(
    buffer: &[u8],
    size: (usize, usize),
    alpha_threshold: u8,
    raw: bool,
    format: ImageFormat,
) -> ImageResult<String> {
    of_image_bytes_with_filter_and_format(
        buffer,
        size,
        alpha_threshold,
        raw,
        FilterType::Nearest,
        format,
    )
}
