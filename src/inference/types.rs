#![allow(dead_code)]
use image::{DynamicImage, ImageBuffer, RgbImage};
use ndarray::{Array3, Array4};

pub struct InferenceProcessor {
    input_image: Vec<Vec<f64>>,
}

impl InferenceProcessor {
    pub fn normalize(
        &self,
        img: &Vec<Vec<f64>>,
        mean: (f64, f64, f64),
        std: (f64, f64, f64),
        size: (u32, u32),
    ) -> Result<Array4<f32>, String> {
        // Convert the image to a DynamicImage for resizing
        let img_buffer: RgbImage =
            ImageBuffer::from_fn(img[0].len() as u32, img.len() as u32, |x, y| {
                let gray_val = img[y as usize][x as usize] as u8;
                image::Rgb([gray_val, gray_val, gray_val])
            });
        let img = DynamicImage::ImageRgb8(img_buffer);

        // Resize the image
        let resized_img = img.resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3);

        // Convert the image to a 2D array of f64
        let img_data: Vec<f64> = resized_img
            .to_rgb8()
            .pixels()
            .flat_map(|p| [p[0] as f64, p[1] as f64, p[2] as f64])
            .collect();

        let img_array = Array3::from_shape_vec((size.1 as usize, size.0 as usize, 3), img_data)
            .map_err(|e| format!("Failed to create array: {}", e))?;

        // Normalize the image
        let max_val = img_array.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let normalized_img_array = img_array.mapv(f64::abs) / max_val.max(1e-6);

        let mut input_tensor = Array4::<f32>::zeros((1, 3, size.1 as usize, size.0 as usize));

        for c in 0..3 {
            for h in 0..size.1 as usize {
                for w in 0..size.0 as usize {
                    let mean_val = match c {
                        0 => mean.0,
                        1 => mean.1,
                        2 => mean.2,
                        _ => unreachable!(),
                    };
                    let std_val = match c {
                        0 => std.0,
                        1 => std.1,
                        2 => std.2,
                        _ => unreachable!(),
                    };
                    input_tensor[[0, c, h, w]] =
                        ((normalized_img_array[[h, w, c]] - mean_val) / std_val) as f32;
                }
            }
        }

        Ok(input_tensor)
    }
}
