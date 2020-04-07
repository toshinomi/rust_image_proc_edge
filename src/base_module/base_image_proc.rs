use image::DynamicImage;

pub trait GoImageProc {
    fn go_image_proc(&mut self, img: &mut DynamicImage);
}