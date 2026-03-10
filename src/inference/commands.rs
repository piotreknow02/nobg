use crate::error::Error;
use crate::inference::process::{read_input_and_resize, run_inference, save_output};
use crate::model::types::RembgModel;

pub fn cli_run(model: String, input: String, output: String) -> Result<(), Error> {
    let model_info =
        RembgModel::find_model(&model).ok_or_else(|| Error::ModelNotFound(model.clone()))?;
    let (image_for_inference, original_img) = read_input_and_resize(&input, model_info.resolution)?;
    let result = run_inference(image_for_inference, model_info)?;
    save_output(result, model_info.resolution, original_img, &output)?;
    Ok(())
}
