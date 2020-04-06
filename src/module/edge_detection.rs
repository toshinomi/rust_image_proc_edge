use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;
use crate::module::common::i64_to_u8;
use crate::module::Pixel;

pub struct ImageManager {}

impl ImageManager {
    pub fn new() -> ImageManager {
        ImageManager {} 
    }
}

pub trait GoImageProc {
    fn go_image_proc(&mut self, img: &mut DynamicImage) -> &mut ImageManager;
}

impl GoImageProc for ImageManager {
    fn go_image_proc(&mut self, img: &mut DynamicImage) -> &mut ImageManager {
        let mask: [[i32; 3]; 3] = [[1,  1, 1], [1, -8, 1], [1,  1, 1]];
        let mask_size = mask.len() as u32;
        let (width, height) = img.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel: Rgba<u8> = img.get_pixel(x, y);

                let mut cal_pixel: Pixel<i64> = Pixel { red: 0, green: 0, blue: 0, alpha: pixel[3] as i64 };
                for y_mask in 0..mask_size {
                    for x_mask in 0..mask_size {
                        if x + x_mask > 0 &&
                        x + x_mask < width &&
                        y + y_mask > 0 &&
                        y + y_mask < height {
                            let pixel2: Rgba<u8> = img.get_pixel(x + x_mask, y + y_mask);
                            let pixel_data: Pixel<u8> = Pixel { red: pixel2[0], green: pixel2[1],  blue: pixel2[2], alpha: pixel2[3] };
                            cal_pixel.red += pixel_data.red as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            cal_pixel.green += pixel_data.green as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                            cal_pixel.blue += pixel_data.blue as i64 * mask[x_mask as usize][y_mask as usize] as i64;
                        }
                    }
                }
                let pixel_data: Pixel<u8> = Pixel { red: pixel[0], green: pixel[1],  blue: pixel[2], alpha: pixel[3] };
                let cal_pixel: Pixel<u8> = Pixel { red: i64_to_u8(cal_pixel.red), green: i64_to_u8(cal_pixel.green),  blue: i64_to_u8(cal_pixel.blue), alpha: pixel_data.alpha };
                let new_color = [cal_pixel.red, cal_pixel.green, cal_pixel.blue, cal_pixel.alpha];
                let pixel: Rgba<u8> = Rgba(new_color);
                img.put_pixel(x, y, pixel);
            }
        }
        self
    }
}