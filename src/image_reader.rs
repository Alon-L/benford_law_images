extern crate image;

pub mod image_reader {
    use super::image::{DynamicImage, GenericImageView, ImageError, Pixels};

    pub struct ImageReader {
        image: DynamicImage,
    }

    impl ImageReader {
        pub fn new(path: &str) -> Result<ImageReader, ImageError> {
            Ok(ImageReader {
                image: image::open(path)?,
            })
        }

        // Returns the image's pixels
        pub fn pixels(&self) -> Pixels<DynamicImage> {
            self.image.pixels()
        }

        // Returns the number of pixels this image contains
        pub fn pixels_size(&self) -> u32 {
            self.image.dimensions().0 * self.image.dimensions().1
        }
    }
}
