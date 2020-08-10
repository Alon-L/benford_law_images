mod image_reader;
mod image_pixel;
mod histogram_writer;

use crate::image_reader::image_reader::ImageReader;
use crate::image_pixel::image_pixel::ImagePixel;
use crate::histogram_writer::histogram::HistogramWriter;
use std::collections::HashMap;

// The path to the image
const PATH: &str = "images/image1.png";

fn main() {
    let reader = ImageReader::new(PATH).expect("Could not read image");

    let mut entries: HashMap<u8, u32> = HashMap::new();
    // Total number of pixels in the image
    let mut total = reader.pixels_size();

    // Iterate over the image's pixels
    // Reduce the pixel's RGB values and retrieve its first digit
    for pixel in reader.pixels() {
        let image_pixel = ImagePixel::new(&pixel.2);
        let reduced = image_pixel.reduce();

        let first_digit = (reduced / 100) as u8;

        *entries.entry(first_digit).or_insert(0) += 1;
    }

    // Vector containing the digits repeatedly for their appearance percentage out of the entire image
    let mut data: Vec<u32> = vec![];

    for (digit, count) in entries {
        // Calculate the percentage for the digit's appearances
        let percentage = (count as f64 / total as f64) * 100 as f64;

        for _ in 0..percentage as u32 {
            data.push(digit as u32);
        }
    }

    let histogram_writer = HistogramWriter::new(PATH, &data);

    // Draw the histogram
    histogram_writer.draw("histogram.png").unwrap();
}
