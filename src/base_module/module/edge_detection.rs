use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;
use crate::base_module::module::common::i64_to_u8;
use crate::base_module::module::common::Pixel;
use crate::base_module::module::common::PixelKind;
use crate::base_module::base_image_proc::GoImageProc;

pub struct ImageManager {}

impl ImageManager {
    pub fn new() -> ImageManager {
        ImageManager {} 
    }
}

impl GoImageProc for ImageManager {
    fn go_image_proc(&mut self, img: &mut DynamicImage) {
        let mask: [[i32; 3]; 3] = [
            [1,  1, 1],
            [1, -8, 1],
            [1,  1, 1]
        ];
        let mask_size = mask.len() as u32;
        let (width, height) = img.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel: Rgba<u8> = img.get_pixel(x, y);

                let mut cal_pixel: Pixel<i64> = Pixel::new(0, 0, 0, pixel[PixelKind::Alpha as usize] as i64);
                for y_mask in 0..mask_size {
                    for x_mask in 0..mask_size {
                        if x + x_mask > 0 &&
                        x + x_mask < width &&
                        y + y_mask > 0 &&
                        y + y_mask < height {
                            let pixel2: Rgba<u8> = img.get_pixel(x + x_mask, y + y_mask);
                            let mut pixel_data: Pixel<u8> = Pixel::new(
                                pixel2[PixelKind::Red as usize],
                                pixel2[PixelKind::Green as usize], 
                                pixel2[PixelKind::Blue as usize],
                                pixel2[PixelKind::Alpha as usize]
                            );
                            let set_value: i64 = cal_pixel.get_red() + pixel_data.get_red().clone() as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            cal_pixel.set_red(set_value);
                            let set_value: i64 = cal_pixel.get_green() + pixel_data.get_green().clone() as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            cal_pixel.set_green(set_value);
                            let set_value: i64 = cal_pixel.get_blue() + pixel_data.get_blue().clone() as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            cal_pixel.set_blue(set_value);
                        }
                    }
                }
                let mut pixel_data: Pixel<u8> = Pixel::new(
                    pixel[PixelKind::Red as usize],
                    pixel[PixelKind::Green as usize], 
                    pixel[PixelKind::Blue as usize],
                    pixel[PixelKind::Alpha as usize]
                );
                let mut cal_pixel: Pixel<u8> = Pixel::new(
                    i64_to_u8(cal_pixel.get_red().clone()),
                    i64_to_u8(cal_pixel.get_green().clone()),
                    i64_to_u8(cal_pixel.get_blue().clone()),
                    pixel_data.get_alpha().clone()
                );
                let new_color = [
                    cal_pixel.get_red().clone(),
                    cal_pixel.get_green().clone(),
                    cal_pixel.get_blue().clone(),
                    cal_pixel.get_alpha().clone()
                ];
                let pixel: Rgba<u8> = Rgba(new_color);
                img.put_pixel(x, y, pixel);
            }
        }
    }
}