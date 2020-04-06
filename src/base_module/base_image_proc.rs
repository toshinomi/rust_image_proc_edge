use image::DynamicImage;

pub struct ImageManager {}

impl ImageManager {
    pub fn new() -> ImageManager {
        ImageManager {} 
    }
}

pub trait GoImageProc {
    fn go_image_proc(&mut self, img: &mut DynamicImage) -> &mut ImageManager;
}