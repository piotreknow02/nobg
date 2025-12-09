use crate::{error::Error, model::commands::get_path};
use onnxruntime::{
    GraphOptimizationLevel, LoggingLevel,
    environment::Environment,
    session::{Session, SessionBuilder},
};
use std::path::Path;

pub fn cli_run(model: String, input: String, output: String) -> Result<(), Error> {
    let image_for_inference = read_input(input);
    let result = run_inference(image_for_inference, model.as_ref());
}

pub fn run_inference(input: Vec<Vec<f64>>, model: &str) -> Result<Vec<Vec<f64>>, Error> {
    let model_path = get_path(model)?;
    let env = Environment::builder()
        .with_log_level(LoggingLevel::Verbose)
        .build()?;

    let session = env
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_model_from_file(model_path)?;

    let output_data = session.run(&[&input])?;

    Ok(output_data)
}

fn read_input(path: String) -> Vec<Vec<f64>> {
    // Implement the logic to read image file into a format suitable inference
    todo!();
}

fn save_output(data: Vec<Vec<f32>>, path: &str) -> Result<(), Error> {
    // Implement the logic to save the output data to a file
    todo!();
    Ok(())
}
