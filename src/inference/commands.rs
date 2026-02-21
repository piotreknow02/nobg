use crate::error::Error;
use crate::model::types::RembgModel;
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use ndarray::{Array3, Array4};
use ort::session::builder::GraphOptimizationLevel;
use ort::value::Tensor;

pub fn cli_run(model: String, input: String, output: String) -> Result<(), Error> {
    let (image_for_inference, original_img) = read_input(&input)?;
    let result = run_inference(image_for_inference, &model)?;
    save_output(result, original_img, &output)?;
    Ok(())
}

pub fn run_inference(input: Array4<f32>, model: &str) -> Result<Array4<f32>, Error> {
    let model_info =
        RembgModel::find_model(model).ok_or_else(|| Error::ModelNotFound(model.to_string()))?;

    let model_path = model_info.get_path()?;

    if !model_path.exists() {
        return Err(Error::ModelNotFound(model.to_string()));
    }

    let mut session = ort::session::Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .commit_from_file(model_path)?;

    let input_tensor = Tensor::from_array(input)?;
    let output_tensors = session.run(ort::inputs![input_tensor])?;

    let output_tensor = &output_tensors[0];
    let output_view = output_tensor.try_extract_array::<f32>()?;
    let output_array: Array4<f32> = output_view
        .into_owned()
        .into_dimensionality()
        .map_err(|e| Error::InferenceError(format!("Failed to convert output shape: {}", e)))?;
    Ok(output_array)
}

fn read_input(path: &str) -> Result<(Array4<f32>, DynamicImage), Error> {
    let img = image::open(path).map_err(|e| Error::ImageError(e))?;
    let original = img.clone();

    let resized_img = img.resize_exact(320, 320, image::imageops::FilterType::Lanczos3);

    let img_data: Vec<f32> = resized_img
        .to_rgb8()
        .pixels()
        .flat_map(|p| [p[0] as f32, p[1] as f32, p[2] as f32])
        .collect();

    let img_array = Array3::from_shape_vec((320, 320, 3), img_data)
        .map_err(|e| Error::InferenceError(format!("Failed to create array: {}", e)))?;

    let normalized_img = img_array.mapv(|x| x / 255.0);

    let mut input_tensor = Array4::<f32>::zeros((1, 3, 320, 320));

    for c in 0..3 {
        for h in 0..320 {
            for w in 0..320 {
                input_tensor[[0, c, h, w]] = normalized_img[[h, w, c]];
            }
        }
    }

    Ok((input_tensor, original))
}

fn save_output(data: Array4<f32>, original: DynamicImage, path: &str) -> Result<(), Error> {
    if !path.to_lowercase().ends_with(".png") {
        return Err(Error::ImageError(image::ImageError::Unsupported(
            image::error::UnsupportedError::from_format_and_kind(
                image::ImageFormat::Jpeg.into(),
                image::error::UnsupportedErrorKind::Color(image::ExtendedColorType::Rgba8),
            ),
        )));
    }

    let mask = data.mapv(|x| x.max(0.0).min(1.0));

    let orig_width = original.width();
    let orig_height = original.height();

    let mask_image = ImageBuffer::from_fn(320, 320, |x, y| {
        let alpha = (mask[[0, 0, y as usize, x as usize]] * 255.0) as u8;
        Rgba([alpha, alpha, alpha, alpha])
    });

    let resized_mask = image::imageops::resize(
        &mask_image,
        orig_width,
        orig_height,
        image::imageops::FilterType::Lanczos3,
    );

    let mut rgba_image: RgbaImage = original.to_rgba8();

    for (mask_pixel, img_pixel) in resized_mask.pixels().zip(rgba_image.pixels_mut()) {
        img_pixel[3] = mask_pixel[0];
    }

    rgba_image.save(path).map_err(|e| Error::ImageError(e))?;

    Ok(())
}
