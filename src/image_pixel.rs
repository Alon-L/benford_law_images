extern crate image;

pub mod image_pixel {
    use super::image::{Pixel, Rgb, Rgba};

    // The maximum sum for the RGB channels for a pixel
    const SUM_MAX: u16 = (255 + 1) * 3;
    // The maximum number the reduced pixel's value is allowed to be
    const RANGE_MAX: u16 = 999;
    // The minimum number the reduced pixel's value is allowed to be
    const RANGE_MIN: u16 = 100;
    const RANGE_DELTA: u16 = RANGE_MAX - RANGE_MIN;

    pub struct ImagePixel {
        rgb: Rgb<u8>,
    }

    // Represents a pixel in an image
    impl ImagePixel {
        pub fn new(pixel: &Rgba<u8>) -> ImagePixel {
            ImagePixel {
                rgb: pixel.to_rgb(),
            }
        }

        // Sums the RGB values of this pixel
        fn sum(&self) -> u16 {
            self.rgb.channels().iter().fold(0, |acc, x| acc + *x as u16)
        }

        // Reduces the pixel's RGB values into a value between 100-999
        pub fn reduce(&self) -> u16 {
            let sum = self.sum();

            let unit_interval = sum as f64 / SUM_MAX as f64;

            let delta = unit_interval * RANGE_DELTA as f64;

            delta as u16 + RANGE_MIN
        }
    }
}
