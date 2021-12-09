use std::path::PathBuf;

use image::imageops::FilterType;

use crate::{of_image, of_image_with_filter};

#[test]
fn it_works() {
    let mut time;
    time = std::time::Instant::now(); println!("{} {:?}", of_image_with_filter(PathBuf::from("test_images/Original/tux.png"), (16, 16), 50, FilterType::CatmullRom).unwrap(), time.elapsed());
    time = std::time::Instant::now(); println!("{} {:?}", of_image_with_filter(PathBuf::from("test_images/Original/tux.png"), (16, 16), 50, FilterType::Gaussian  ).unwrap(), time.elapsed());
    time = std::time::Instant::now(); println!("{} {:?}", of_image_with_filter(PathBuf::from("test_images/Original/tux.png"), (16, 16), 50, FilterType::Lanczos3  ).unwrap(), time.elapsed());
    time = std::time::Instant::now(); println!("{} {:?}", of_image            (PathBuf::from("test_images/Original/tux.png"), (16, 16), 50                        ).unwrap(), time.elapsed());
    time = std::time::Instant::now(); println!("{} {:?}", of_image_with_filter(PathBuf::from("test_images/Original/tux.png"), (16, 16), 50, FilterType::Triangle  ).unwrap(), time.elapsed());
}
