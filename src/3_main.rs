use image::{DynamicImage, ImageBuffer, Rgba};
fn load_image(filepath: &str) -> Result<DynamicImage, image::ImageError> {
    image::open(filepath)
}
fn rotate_image_90_clockwise(img: &ImageBuffer<Rgba<u8>, Vec<u8>>,) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut new_img = ImageBuffer::new(height, width); // new image dimensions are swapped
    img.enumerate_pixels().for_each(|(x, y, pixel)| {
        let new_x = height - y - 1; // new x is (height-y-1)
        let new_y = x; // new y is the old x
        new_img.put_pixel(new_x, new_y, *pixel);
    });
    new_img
}
fn main() {
    println!("Image Processing - Enumerating Pixels");
    let img_path = "assets/sample_img.png";
    let img = load_image(img_path).expect("Failed to load image");
    let rotated_img = rotate_image_90_clockwise(&img.to_rgba8()); // rotate 90 degrees clockwise
    rotated_img
        .save("assets/processed_img.png")
        .expect("Failed to save processed image.");
}