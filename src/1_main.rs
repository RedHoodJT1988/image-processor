use image::{open, DynamicImage, imageops::FilterType::Lanczos3, GenericImageView};
fn resize_image(path: &str, width: u32, height: u32) -> DynamicImage {
    let img = open(path).expect("Failed to open image");
    img.resize(width, height, FilterType::Lanczos3)
}
fn save_image(img: &DynamicImage, output_path: &str) {
    img.save_with_format(output_path, image::ImageFormat::Png).expect("Failed to save image");
}
fn rotate_image(path: &str, degrees: u32) -> DynamicImage {
    let img = open(path).expect("Failed to load image");
    match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => {
            eprintln!("Unsupported rotation angle. Please give 90, 180, 270.");
            img
        }
    }
}
fn main() {
    println!("Image Processing");
    let rotated_img = rotate_image(
        "D:\\image_processor\\assets\\sample_img.jpg",
        90
    );
    save_image(
        &rotated_img,
        "D:\\image_processor\\assets\\sample_img_ROTATED.png"
    );
    // let resized_image = resize_image(
    //     "D:\\image_processor\\assets\\sample_img.jpg",
    //     512,
    //     512
    // );
    
    // save_image(
    //     &resized_image,
    //     "D:\\image_processor\\assets\\sample_img_RESIZED.png"
    // );
}