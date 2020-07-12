use image::DynamicImage;

pub trait GoImageProcessing {
    fn go_image_processing(&mut self, img: &mut DynamicImage);
}