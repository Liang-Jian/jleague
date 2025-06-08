// use std::fs;
// use std::path::Path;
// 
// use image::{Rgb, RgbImage};
// 
// fn save_image<F>(func: F, filename: &str)
// where
//     F: Fn() -> RgbImage,
// {
//     let image = func();
// 
//     let dir = Path::new("images");
//     if !dir.exists() {
//         fs::create_dir_all(dir).expect("Failed to create directory");
//     }
// 
//     let filepath = dir.join(filename);
// 
//     // 正确写法：直接传路径而不是 BufWriter
//     image
//         .save_with_format(&filepath, image::ImageFormat::Png)
//         .expect("Failed to save image");
// 
//     println!("Saved image {}", filename);
// }
// 
// fn generate_image(width: u32, height: u32, color: (u8, u8, u8)) -> RgbImage {
//     let mut img = RgbImage::new(width, height);
//     for pixel in img.pixels_mut() {
//         *pixel = Rgb([color.0, color.1, color.2]);
//     }
//     img
// }
// 
// fn main() {
//     let width = 640;
//     let height = 480;
//     let color = (255, 255, 255); // 白色
// 
//     save_image(|| generate_image(width, height, color), "image.png");
// }

mod config;
mod db;
mod fetch;
mod parse;