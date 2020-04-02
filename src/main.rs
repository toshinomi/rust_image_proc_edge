extern crate image;

use image::GenericImageView;
use image::GenericImage;
use image::DynamicImage;
use image::Rgba;
use std::io;

fn main() {
    println!("image directory : ");
    let mut directory = String::new();
    io::stdin().read_line(&mut directory).expect("Failed to read line");
    directory.pop();
    let input_image = directory.to_string() + "/input.jpg";
    let output_image = directory.to_string() + "/output.jpg";
    let mut img: DynamicImage = image::open(input_image).unwrap();

    println!("Image Processing Start!");

    edge_detection(&mut img);

    img.save(&output_image).unwrap();

    println!("Image Processing End!");
}

fn edge_detection(img: &mut DynamicImage) {
    let mask: [[i32; 3]; 3] = [[1,  1, 1], [1, -8, 1], [1,  1, 1]];
    let mask_size = mask.len() as u32;
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel: Rgba<u8> = img.get_pixel(x, y);

            let mut cal_r: i32 = 0;
            let mut cal_g: i32 = 0;
            let mut cal_b: i32 = 0;
            let cal_a: u8 = pixel[3];
            for y_mask in 0..mask_size {
                for x_mask in 0..mask_size {
                    if x + x_mask > 0 &&
                       x + x_mask < width &&
                       y + y_mask > 0 &&
                       y + y_mask < height {
                           let pixel2: Rgba<u8> = img.get_pixel(x + x_mask, y + y_mask);
                           cal_r += pixel2[0] as i32 * mask[x_mask as usize][y_mask as usize];
                           cal_g += pixel2[1] as i32 * mask[x_mask as usize][y_mask as usize];
                           cal_b += pixel2[2] as i32 * mask[x_mask as usize][y_mask as usize];
                    }
                }
            }
            let cal_r: u8 = i32_to_u8(cal_r) as u8;
            let cal_g: u8 = i32_to_u8(cal_g) as u8;
            let cal_b: u8 = i32_to_u8(cal_b) as u8;
            let new_color = [cal_r, cal_g, cal_b, cal_a];
            let pixel: Rgba<u8> = Rgba(new_color);
            img.put_pixel(x, y, pixel);
        }
    }
}

fn i32_to_u8(value: i32) -> u8 {
    let cnv_value: u8;
    if value > 255 {
        cnv_value = 255;
    }
    else if value < 0 {
        cnv_value = 0;
    }
    else {
        cnv_value = value as u8;
    }
    cnv_value
}