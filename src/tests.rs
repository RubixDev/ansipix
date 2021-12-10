use std::path::PathBuf;

use image::imageops::FilterType;

use crate::{of_image, of_image_with_filter};

#[test]
fn filter_types() {
    let mut time;
    time = std::time::Instant::now(); println!("{} {:?}", of_image_with_filter(PathBuf::from("test_images/Original/tux.png"), (16, 16), 50, false, FilterType::CatmullRom).unwrap(), time.elapsed());
    time = std::time::Instant::now(); println!("{} {:?}", of_image_with_filter(PathBuf::from("test_images/Original/tux.png"), (16, 16), 50, false, FilterType::Gaussian  ).unwrap(), time.elapsed());
    time = std::time::Instant::now(); println!("{} {:?}", of_image_with_filter(PathBuf::from("test_images/Original/tux.png"), (16, 16), 50, false, FilterType::Lanczos3  ).unwrap(), time.elapsed());
    time = std::time::Instant::now(); println!("{} {:?}", of_image            (PathBuf::from("test_images/Original/tux.png"), (16, 16), 50, false                        ).unwrap(), time.elapsed());
    time = std::time::Instant::now(); println!("{} {:?}", of_image_with_filter(PathBuf::from("test_images/Original/tux.png"), (16, 16), 50, false, FilterType::Triangle  ).unwrap(), time.elapsed());
}

#[test]
fn raw_output() {
    println!("{}", of_image(PathBuf::from("test_images/Original/tux.png"), (8, 8), 10, false).unwrap());
    println!("{}", of_image(PathBuf::from("test_images/Original/tux.png"), (8, 8), 10, true).unwrap());
}
