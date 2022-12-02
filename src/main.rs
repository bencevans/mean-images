use image::GenericImageView;
use rayon::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let root_path = PathBuf::from(&args[1]);

    let walker = WalkDir::new(root_path).into_iter();

    let results = walker.par_bridge().into_par_iter().filter_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();

        let Ok(img) = image::open(path) else {
            return None;
        };

        let mut total_pixels: u32 = 0;
        let mut total_red: u32 = 0;
        let mut total_green: u32 = 0;
        let mut total_blue: u32 = 0;

        for (_, _, rgba) in img.pixels() {
            total_pixels += 1;
            total_red += rgba[0] as u32;
            total_green += rgba[1] as u32;
            total_blue += rgba[2] as u32;
        }

        if total_pixels == 0 {
            return None;
        }

        let avg_red = total_red / total_pixels;
        let avg_green = total_green / total_pixels;
        let avg_blue = total_blue / total_pixels;

        Some((avg_red, avg_green, avg_blue))
    });

    let mut total_red = 0;
    let mut total_green = 0;
    let mut total_blue = 0;
    let mut total_images = 0;

    results
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(r, g, b)| {
            total_red += r;
            total_green += g;
            total_blue += b;
            total_images += 1;
        });

    let avg_red = total_red / total_images;
    let avg_green = total_green / total_images;
    let avg_blue = total_blue / total_images;

    println!(
        "R: {} ({}) G: {} ({}) B: {} ({})",
        avg_red,
        avg_red as f32 / 255_f32,
        avg_green,
        avg_green as f32 / 255_f32,
        avg_blue,
        avg_blue as f32 / 255_f32
    );
}
