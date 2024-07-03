use image::{open, GenericImageView, ImageFormat, DynamicImage};

fn main() {
  let img = open("H:\\rust\\image-processor\\assets\\powerranges.jpg");

  // let (width, height) = img.dimensions();

  // println!("Width: {}, Height: {}", width, height);

  // saving with specific format png
  img.save_with_format("H:\\rust\\image-processor\\assets\\powerranges.jpg", ImageFormat::Png)
    .epxect("Failed to save image as png");

  img.save_with_format("H:\\rust\\image-processor\\assets\\powerranges.jpg", ImageFormat::WebP)
    .expect("Failed to save image as webp");
}