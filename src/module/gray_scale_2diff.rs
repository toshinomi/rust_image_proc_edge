use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;
use crate::base_module::common::PixelSumData;
use crate::base_module::common::Pixel;
use crate::base_module::common::PixelKind;
use crate::base_module::base_image_proc::GoImageProcessing;

pub struct GrayScale2Diff {}

impl GrayScale2Diff {
    pub fn new() -> GrayScale2Diff {
        GrayScale2Diff {} 
    }
}

impl GoImageProcessing for GrayScale2Diff {
    fn go_image_processing(&mut self, image: &mut DynamicImage) {
        let mask: [[i32; 3]; 3] = [
            [1,  1, 1],
            [1, -8, 1],
            [1,  1, 1]
        ];
        let mask_size = mask.len() as u32;
        let (width, height) = image.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel: Rgba<u8> = image.get_pixel(x, y);                
                let mut average_gray_scale: f64 = 0.0f64;
                for y_mask in 0..mask_size {
                    for x_mask in 0..mask_size {
                        if x + x_mask > 0 &&
                        x + x_mask < width &&
                        y + y_mask > 0 &&
                        y + y_mask < height {
                            let pixel_mask_area: Rgba<u8> = image.get_pixel(x + x_mask, y + y_mask);
                            let mut pixel_data: Pixel<u8> = Pixel::new(
                                pixel_mask_area[PixelKind::Red as usize],
                                pixel_mask_area[PixelKind::Green as usize], 
                                pixel_mask_area[PixelKind::Blue as usize],
                                pixel_mask_area[PixelKind::Alpha as usize]
                            );
                            let red: i64 = pixel_data.get_red().clone() as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            let green: i64 = pixel_data.get_green().clone() as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            let blue: i64 = pixel_data.get_blue().clone() as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                        
                            let gray: f64 = (blue + green + red) as f64 / 3.0f64;
                            average_gray_scale = (average_gray_scale + gray) / 2.0f64;
                        }
                    }
                }
                let mut convert_data: u8 = 0;
                if average_gray_scale > 255.0f64 {
                    convert_data = 255;
                }
                else if average_gray_scale < 0.0f64 {
                    convert_data = 0;
                }
                else {
                    convert_data = average_gray_scale.clone() as u8;
                }
                let new_color = [
                    convert_data,
                    convert_data,
                    convert_data,
                    pixel[PixelKind::Alpha as usize]
                ];
                let pixel: Rgba<u8> = Rgba(new_color);
                image.put_pixel(x, y, pixel);
            }
        }
    }
}