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

        let mut total_pixels: f32 = 0.0;
        let mut total_red: f32 = 0.0;
        let mut total_green: f32 = 0.0;
        let mut total_blue: f32 = 0.0;

        for (_, _, rgba) in img.pixels() {
            total_pixels += 1.0;
            total_red += rgba[0] as f32;
            total_green += rgba[1] as f32;
            total_blue += rgba[2] as f32;
        }

        if total_pixels == 0.0 {
            return None;
        }

        let avg_red = total_red / total_pixels;
        let avg_green = total_green / total_pixels;
        let avg_blue = total_blue / total_pixels;

        Some((avg_red, avg_green, avg_blue))
    });

    let results = results.collect::<Vec<_>>();

    let total_images = results.len() as f32;

    let mut avg_red: f32 = 0.0;
    let mut avg_green: f32 = 0.0;
    let mut avg_blue: f32 = 0.0;

    results.into_iter().for_each(|(r, g, b)| {
        avg_red += r / total_images;
        avg_green += g / total_images;
        avg_blue += b / total_images;
    });

    println!("R: {} G: {} B: {}", avg_red, avg_green, avg_blue,);
}
