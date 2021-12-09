use std::path::PathBuf;
use image::{imageops::FilterType, ImageResult, GenericImageView};

const TOP_HALF:    &str = "\u{2580}";
const BOTTOM_HALF: &str = "\u{2584}";

pub fn of_image_with_filter(file: PathBuf, size: (usize, usize), alpha_threshold: u8, resize_filter: FilterType) -> ImageResult<String> {
    let img = image::open(&file)?;
    let mut pixels: Vec<Vec<[u8; 4]>> = vec![];
    for (x, y, pix) in img.resize(size.0 as u32, size.1 as u32, resize_filter).pixels() {
        if x == 0 { pixels.push(vec![]); }
        pixels[y as usize].push(pix.0);
    }

    let mut out: String = String::new();
    for line in (0..pixels.len()).filter(|index| index % 2 == 0) {
        for char in 0..pixels[line].len() {
            let top_pix: [u8; 4] = pixels[line][char];
            let bot_pix: [u8; 4] = if line + 1 >= pixels.len() { [0; 4] } else { pixels[line + 1][char] };
            let top_invis: bool = top_pix[3] < alpha_threshold;
            let bot_invis: bool = bot_pix[3] < alpha_threshold;

            if top_invis && bot_invis {
                out += " ";
            } else if top_invis && !bot_invis {
                out += format!("\x1b[38;2;{};{};{}m{}\x1b[0m", bot_pix[0], bot_pix[1], bot_pix[2], BOTTOM_HALF).as_str();
            } else if !top_invis && bot_invis {
                out += format!("\x1b[38;2;{};{};{}m{}\x1b[0m", top_pix[0], top_pix[1], top_pix[2], TOP_HALF).as_str();
            } else {
                out += format!(
                    "\x1b[38;2;{};{};{};48;2;{};{};{}m{}\x1b[0m",
                    bot_pix[0], bot_pix[1], bot_pix[2],
                    top_pix[0], top_pix[1], top_pix[2],
                    BOTTOM_HALF
                ).as_str();
            }
        }
        out += "\n";
    }
    return Ok(out);
}

pub fn of_image(file: PathBuf, size: (usize, usize), alpha_threshold: u8) -> ImageResult<String> {
    of_image_with_filter(file, size, alpha_threshold, FilterType::Nearest)
}

#[cfg(test)]
mod tests;
