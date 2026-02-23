use ort::session::builder::GraphOptimizationLevel;

use crate::inference::error::Error;

pub fn get_backend_name() -> &'static str {
    #[cfg(feature = "coreml")]
    {
        "CoreML (Metal)"
    }
    #[cfg(feature = "cuda")]
    {
        "CUDA"
    }
    #[cfg(feature = "rocm")]
    {
        "ROCm"
    }
    #[cfg(feature = "directml")]
    {
        "DirectML"
    }
    #[cfg(not(any(
        feature = "cuda",
        feature = "rocm",
        feature = "coreml",
        feature = "directml"
    )))]
    {
        "CPU"
    }
}

pub fn build_session(
    model_path: &std::path::Path,
    backend: &str,
) -> Result<ort::session::Session, Error> {
    let result = get_session(model_path);

    match result {
        Ok(session) => Ok(session),
        Err(e) => {
            eprintln!(
                "Warning: Failed to initialize {} backend: {}, falling back to CPU",
                backend, e
            );
            let fallback_session = get_session_cpu_only(model_path)?;
            Ok(fallback_session)
        }
    }
}

#[cfg(feature = "coreml")]
fn get_session(model_path: &std::path::Path) -> Result<ort::session::Session, ort::Error> {
    use ort::execution_providers::CoreMLExecutionProvider;
    return ort::session::Session::builder()?
        .with_execution_providers([CoreMLExecutionProvider::default().build()])?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .commit_from_file(model_path);
}

#[cfg(feature = "cuda")]
fn get_session(model_path: &std::path::Path) -> Result<ort::session::Session, ort::Error> {
    use ort::execution_providers::CUDAExecutionProvider;
    return ort::session::Session::builder()?
        .with_execution_providers([CUDAExecutionProvider::default().build()])?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .commit_from_file(model_path);
}

#[cfg(feature = "rocm")]
fn get_session(model_path: &std::path::Path) -> Result<ort::session::Session, ort::Error> {
    use ort::execution_providers::ROCmExecutionProvider;
    return ort::session::Session::builder()?
        .with_execution_providers([ROCmExecutionProvider::default().build()])?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .commit_from_file(model_path);
}

#[cfg(feature = "directml")]
fn get_session(model_path: &std::path::Path) -> Result<ort::session::Session, ort::Error> {
    use ort::execution_providers::DirectMLExecutionProvider;
    return ort::session::Session::builder()?
        .with_execution_providers([DirectMLExecutionProvider::default().build()])?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .commit_from_file(model_path);
}

// TODO: maybe add more backends some day...

fn get_session_cpu_only(model_path: &std::path::Path) -> Result<ort::session::Session, ort::Error> {
    ort::session::Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .commit_from_file(model_path)
}

#[cfg(not(any(
    feature = "cuda",
    feature = "rocm",
    feature = "coreml",
    feature = "directml"
)))]
fn get_session(model_path: &std::path::Path) -> Result<ort::session::Session, ort::Error> {
    get_session_cpu_only(model_path)
}
