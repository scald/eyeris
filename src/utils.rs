use crate::errors::ProcessorError;
use image::{DynamicImage, ImageBuffer, Rgb};
use rayon::prelude::*;

pub fn enhance_image(img: &DynamicImage) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, ProcessorError> {
    let rgb_image = img.to_rgb8();
    let width = rgb_image.width() as usize;
    let height = rgb_image.height() as usize;

    let enhanced_pixels: Vec<_> = rgb_image
        .chunks_exact(width * 3)
        .enumerate()
        .par_bridge()
        .flat_map(|(y, row)| {
            (0..width)
                .map(move |x| {
                    let pixel = Rgb([
                        (row[x * 3] as f32 * 1.1).min(255.0) as u8,
                        (row[x * 3 + 1] as f32 * 1.1).min(255.0) as u8,
                        (row[x * 3 + 2] as f32 * 1.1).min(255.0) as u8,
                    ]);
                    (x as u32, y as u32, pixel)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut enhanced = ImageBuffer::new(width as u32, height as u32);
    for (x, y, pixel) in enhanced_pixels {
        enhanced.put_pixel(x, y, pixel);
    }

    Ok(enhanced)
}
