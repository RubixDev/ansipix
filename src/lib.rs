#![doc = include_str!("../README.md")]

use image::{imageops::FilterType, DynamicImage, GenericImageView};

const TOP_HALF: &str = "\u{2580}";
const BOTTOM_HALF: &str = "\u{2584}";

/// Convert a [`DynamicImage`] to a [`String`] with [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code).
///
/// ## Params
/// - `image`: [`DynamicImage`] - The image to convert.
/// - `size`: `(usize, usize)` - The maximum size of the resized image as a `(width, height)` tuple.
/// - `alpha_threshold`: [`u8`] - Minimum alpha value of a pixel for it to be shown. `0` for no transparent background.
/// - `raw`: [`bool`] - Whether to escape the escape sequences.
///
/// ## Returns
/// A [`String`] containing the image.
pub fn of_image(
    image: &DynamicImage,
    size: (usize, usize),
    alpha_threshold: u8,
    raw: bool,
) -> String {
    of_image_with_filter(image, size, alpha_threshold, raw, FilterType::Nearest)
}

/// Convert a [`DynamicImage`] to a [`String`] with [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code) given a resize filter.
///
/// ## Params
/// - `image`: [`DynamicImage`] - The image to convert.
/// - `size`: `(usize, usize)` - The maximum size of the resized image as a `(width, height)` tuple.
/// - `alpha_threshold`: [`u8`] - Minimum alpha value of a pixel for it to be shown. `0` for no transparent background.
/// - `raw`: [`bool`] - Whether to escape the escape sequences.
/// - `resize_filter`: [`FilterType`] - The filter to be used for resizing the image.
///
/// ## Returns
/// A [`String`] containing the image.
pub fn of_image_with_filter(
    image: &DynamicImage,
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
    out
}
