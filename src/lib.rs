use image::{buffer::ConvertBuffer, ImageBuffer, Luma};
use wasm_bindgen::prelude::*;
use image::GrayImage;

#[wasm_bindgen]
pub fn apply_grayscale(input: &[u8], width: u32, height: u32) -> Vec<u8> {
    let img: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, input.to_vec()).unwrap();
    let gray_image: GrayImage = img.convert();
    gray_image.into_raw()
}
