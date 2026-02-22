use crate::error::Error;
use crate::inference::process::{read_input, run_inference, save_output};

pub fn cli_run(model: String, input: String, output: String) -> Result<(), Error> {
    let (image_for_inference, original_img) = read_input(&input)?;
    let result = run_inference(image_for_inference, &model)?;
    save_output(result, original_img, &output)?;
    Ok(())
}
