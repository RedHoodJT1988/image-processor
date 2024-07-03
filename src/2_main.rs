use image::{open, DynamicImage, imageops::FilterType, ImageFormat, imageops::crop_imm};
use image::GenericImageView; // Required for the dimensions() method
fn save_image(img: &DynamicImage, output_path: &str) {
    // Specify the format directly in the save method
    img.save_with_format(output_path, ImageFormat::Png).expect("Failed to save image");
}
fn resize_image_maintain_ratio(path: &str, new_width: Option<u32>, new_height: Option<u32>) -> DynamicImage {
    let img = open(path).expect("Failed to open image");
    let (width, height) = img.dimensions();
    // calculate new dimensions while maintaining the aspect ratio
    let ratio = width as f32 / height as f32;
    let (resize_width, resize_height) = match(new_width, new_height) {
        (Some(w), None) => (w, (w as f32 / ratio).round() as u32),
        (None, Some(h)) => ((h as f32 * ratio).round() as u32, h),
        (Some(w), Some(h)) => (w, h),    // if both dimensions are specified use them as is
        (None, None) => (width, height), // if no dimensions are specified use the original dimensions
    };
    img.resize(resize_width, resize_height, FilterType::Lanczos3)
}
fn crop_image(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    let cropped_img = crop_imm(img, x, y, width, height);
    DynamicImage::ImageRgba8(cropped_img.to_image())
}
fn main() {
    println!("Image Processing");
    let img = open("D:\\image_processor\\assets\\sample_img.jpg").expect("Failed to open image");
    let cropped_img = crop_image(&img, 100, 2000, 2500, 2500);
    save_image(
        &cropped_img,
        "D:\\image_processor\\assets\\cropped_image.png"
    );    
    // let resized_image = resize_image_maintain_ratio(
    //     "D:\\image_processor\\assets\\sample_img.jpg",
    //     Some(800),
    //     None
    // );
    // save_image(
    //     &resized_image,
    //     "D:\\image_processor\\assets\\sample_img_RE_DI.png"
    // );
}