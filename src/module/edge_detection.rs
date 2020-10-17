use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;
use crate::base_module::common::PixelSumData;
use crate::base_module::common::Pixel;
use crate::base_module::common::PixelKind;
use crate::base_module::base_image_proc::GoImageProcessing;

pub struct EdgeDetection {}

impl EdgeDetection {
    pub fn new() -> EdgeDetection {
        EdgeDetection {} 
    }
}

impl GoImageProcessing for EdgeDetection {
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

                let mut total_pixel_data: Pixel<i64> = Pixel::new(0, 0, 0, pixel[PixelKind::Alpha as usize] as i64);
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
                            let set_value: i64 = total_pixel_data.get_red() + pixel_data.get_red().clone() as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            total_pixel_data.set_red(set_value);
                            let set_value: i64 = total_pixel_data.get_green() + pixel_data.get_green().clone() as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            total_pixel_data.set_green(set_value);
                            let set_value: i64 = total_pixel_data.get_blue() + pixel_data.get_blue().clone() as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            total_pixel_data.set_blue(set_value);
                        }
                    }
                }
                let mut pixel_data: Pixel<u8> = Pixel::new(
                    pixel[PixelKind::Red as usize],
                    pixel[PixelKind::Green as usize], 
                    pixel[PixelKind::Blue as usize],
                    pixel[PixelKind::Alpha as usize]
                );
                let mut total_pixel_data: Pixel<u8> = Pixel::new(
                    PixelSumData::to_u8(total_pixel_data.get_red().clone()),
                    PixelSumData::to_u8(total_pixel_data.get_green().clone()),
                    PixelSumData::to_u8(total_pixel_data.get_blue().clone()),
                    pixel_data.get_alpha().clone()
                );
                let new_color = [
                    total_pixel_data.get_red().clone(),
                    total_pixel_data.get_green().clone(),
                    total_pixel_data.get_blue().clone(),
                    total_pixel_data.get_alpha().clone()
                ];
                let pixel: Rgba<u8> = Rgba(new_color);
                image.put_pixel(x, y, pixel);
            }
        }
    }
}