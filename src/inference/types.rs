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
        ) -> Result<Array3<f32>, String> {
            // Convert the image to a DynamicImage for resizing
            let img_buffer = ImageBuffer::from_fn(img[0].len() as u32, img.len() as u32, |x, y| {
                Luma([img[y as usize][x as usize] as u8])
            });
            let img = DynamicImage::ImageRgb8(img_buffer);

            // Resize the image
            let resized_img = img.resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3);

            // Convert the image to a 2D array of f64
            let img_array: Array3<f64> = resized_img.pixels().map(|p| p[0] as f64).collect::<Vec<_>>()
                .chunks_exact(3)
                .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                .into_shape((size.1 as usize, size.0 as usize, 3))
                .map_err(|e| format!("Failed to create array: {}", e))?;


            // Normalize the image
            let max_val = img_array.max().unwrap_or(1e-6);
            let normalized_img_array = img_array.mapv(f64::abs) / max_val;

            let tmp_img_array = Array3::<f32>::zeros((size.1 as usize, size.0 as usize, 3));
            let mut tmp_img = tmp_img_array.index_axis_mut(ndarray::Axis(0), 0);

            for (i, channel) in normalized_img_array.axis_iter(ndarray::Axis(2)).enumerate() {
                let tmp_channel = channel.mapv(|x| ((x - mean[i]) / std[i]).to_f32().unwrap());
                tmp_img.slice_mut(s![..; tmp_channel.len()]).assign(&tmp_channel);
            }

            // Transpose the array
            Ok(tmp_img_array.reversed_axes()[ndarray::Axis(0), ..].into_shape((1, 3, size.1 as usize, size.0 as usize))?)
    }
}
