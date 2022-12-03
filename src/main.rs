use image::GenericImageView;
use num_bigint::BigUint;
use num_traits::Zero;
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

    let mut total_red: BigUint = Zero::zero();
    let mut total_green: BigUint = Zero::zero();
    let mut total_blue: BigUint = Zero::zero();
    let mut total_images: BigUint = Zero::zero();

    results
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(r, g, b)| {
            total_red += r;
            total_green += g;
            total_blue += b;
            total_images += 1u32;
        });

    let avg_red = total_red / total_images.clone();
    let avg_green = total_green / total_images.clone();
    let avg_blue = total_blue / total_images;

    println!("R: {} G: {} B: {}", avg_red, avg_green, avg_blue,);
}
