extern crate image;

use image::{imageops::FilterType, GenericImageView};
use reqwest::IntoUrl;
use std::{ffi::OsStr, fs::File, io::Write, path::Path};
use tempfile::tempfile;

/// The set of characters to use in an ASCII image. The positioning
/// of characters in the array is important because that position
/// is how they are mapped based on the brightness of a pixel.
const CHARACTER_SET: [&str; 11] = [" ", "'", ",", ".", ":", ";", "/", "O", "0", "#", "@"];

/// Image dimension with a width and a height.
pub struct Dimension {
    width: u32,
    height: u32,
}

impl Dimension {
    /// Create a new dimension pair.
    pub fn new(width: u32, height: u32) -> Self {
        Dimension { width, height }
    }

    /// Get the width value of this dimension.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height value of this dimension.
    pub fn height(&self) -> u32 {
        self.height
    }
}

/// Opens an image at a path and turns it into an ASCII string.
/// If the image is larger than the given dimensions, it will be
/// resized to those dimensions. If `None` is passed, it will
/// use a width and height of 250.
///
/// # Example
///
/// ```rust
/// use asciifyer::{convert_to_ascii, Dimension};
///
/// // Scale the image down by half
/// let dimensions = Dimension::new(300, 300);
/// let ascii_image = convert_to_ascii("./image1.png", Some(dimensions));
/// println!("{}", ascii_image);
/// ```
pub fn convert_to_ascii<S: AsRef<OsStr> + ?Sized>(
    path: &S,
    dimensions: Option<Dimension>,
) -> String {
    let path = Path::new(path);
    let mut art = String::new();

    // TODO: Handle this failing
    if let Ok(mut image) = image::open(&path) {
        let mut last_y = 0;
        let dimensions = dimensions.unwrap_or(Dimension {
            width: 250,
            height: 250,
        });

        // Resize if needed
        if image.width() > dimensions.width() || image.height() > dimensions.height() {
            image = image.resize(dimensions.width(), dimensions.height(), FilterType::Nearest);
        }

        for pixel in image.pixels() {
            // Check the y-value of the pixel to see if we need to add a newline
            if last_y != pixel.1 {
                art.push_str("\n");
                last_y = pixel.1;
            }

            let rgba = pixel.2;

            // Calculate the brightness using the RGB values of the pixel
            // Formula taken from: https://stackoverflow.com/questions/596216/formula-to-determine-brightness-of-rgb-color
            let brightness: f64 = ((0.2126 * rgba[0] as f64)
                + (0.7152 * rgba[0] as f64)
                + (0.0722 * rgba[0] as f64)) as f64;
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
