extern crate image;

use image::{imageops::FilterType, GenericImageView};
use reqwest::IntoUrl;
use std::{ffi::OsStr, fs::File, io::Write, path::Path};
use tempfile::tempfile;

/// The set of characters to use in an ASCII image. The positioning
/// of characters in the array is important because that position
/// is how they are mapped based on the brightness of a pixel.
const CHARACTER_SET: [&str; 11] = ["@", "#", "0", "O", "L", ";", ":", ".", ",", "'", " "];

/// Opens an image at a path and turns it into an ASCII string.
/// Images will be scaled down by the amount provided before being
/// turned into an ASCII image.
///
/// # Example
///
/// ```rust
/// use asciifyer::convert_to_ascii;
///
/// // Scale the image down by half
/// let ascii_image = convert_to_ascii("./image1.png", 2);
/// println!("{}", ascii_image);
/// ```
pub fn convert_to_ascii<S: AsRef<OsStr> + ?Sized>(path: &S, scale: Option<u32>) -> String {
    let path = Path::new(path);
    let mut art = String::new();

    // TODO: Handle this failing
    if let Ok(image) = image::open(&path) {
        let mut last_y = 0;
        let resolution = scale.unwrap_or(1);

        let image = image.resize(
            image.width() / resolution,
            image.height() / resolution,
            FilterType::Nearest,
        );

        for pixel in image.pixels() {
            // Check the y-value of the pixel to see if we need to add a newline
            if last_y != pixel.1 {
                art.push_str("\n");
                last_y = pixel.1;
            }

            let rgba = pixel.2;

            // Calculate the brightness using the RGB values of the pixel
            let brightness: f64 = ((rgba[0] as u64 + rgba[1] as u64 + rgba[2] as u64) / 3) as f64;
            let position =
                ((brightness / 255.0) * (CHARACTER_SET.len() - 1) as f64).round() as usize;
            art.push_str(CHARACTER_SET[position])
        }
    }

    art
}

/// Downloads an image from a remote location, returning the temporary file
/// with the image. The temporary file will automatically be removed when
/// the last handle is closed.
pub async fn fetch_remote_image<T: IntoUrl>(url: T) -> Result<File, Box<dyn std::error::Error>> {
    let bytes = reqwest::get(url).await?.bytes().await?;

    let mut out_file = tempfile()?;
    out_file.write_all(&bytes)?;

    Ok(out_file)
}
