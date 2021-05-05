extern crate image;

use image::DynamicImage;
use std::io;
use rust_image_proc_edge::module::edge_detection;
use rust_image_proc_edge::module::binarization;
use rust_image_proc_edge::module::color_reversal;
use rust_image_proc_edge::base_module::base_image_proc::GoImageProcessing;

fn main() {
    println!("image directory : ");
    let mut directory = String::new();
    io::stdin().read_line(&mut directory).expect("Failed to read line");
    directory.pop();
    let input_image = directory.to_string() + "/input.jpg";
    let output_image = directory.to_string() + "/output.jpg";
    let mut image: DynamicImage = image::open(input_image).unwrap();

    println!("Image Processing Start!");

    let mut edge = edge_detection::EdgeDetection::new();
    edge.go_image_processing(&mut image);
    // let mut binarization = binarization::Binarization::new(127);
    // binarization.go_image_processing(&mut image);
    // let mut color_reversal = color_reversal::ColorReversal::new();
    // color_reversal.go_image_processing(&mut image);

    image.save(&output_image).unwrap();

    println!("Image Processing End!");
}