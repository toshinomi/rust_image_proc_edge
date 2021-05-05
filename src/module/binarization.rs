use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;
// use crate::base_module::common::PixelSumData;
// use crate::base_module::common::Pixel;
use crate::base_module::common::PixelKind;
use crate::base_module::base_image_proc::GoImageProcessing;

pub struct Binarization {
    thresh: u8
}

impl Binarization {
    pub fn new(_thresh: u8) -> Binarization {
        Binarization { thresh: _thresh }
    }
}

impl GoImageProcessing for Binarization {
    fn go_image_processing(&mut self, image: &mut DynamicImage) {
        let (width, height) = image.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel: Rgba<u8> = image.get_pixel(x, y);
                let gray_scale: u64 = (pixel[PixelKind::Red as usize] as u64 + pixel[PixelKind::Green as usize] as u64 + pixel[PixelKind::Blue as usize] as u64) / 3;

                let binarization: u8 = if gray_scale as u8 >= self.thresh { 255u8 } else { 0u8 };
                let new_color = [
                    binarization,
                    binarization,
                    binarization,
                    pixel[PixelKind::Alpha as usize]
                ];
                let pixel: Rgba<u8> = Rgba(new_color);
                image.put_pixel(x, y, pixel);
            }
        }
    }
}