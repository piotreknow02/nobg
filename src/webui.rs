use axum::{
    Router,
    extract::Multipart,
    routing::{get, post},
};
use axum_embed::ServeEmbed;
use base64::Engine;
use bytes::Bytes;
use std::net::SocketAddr;

use crate::inference::process::{read_input_and_resize, run_inference, save_output};
use crate::model::registry::MODELS;
use crate::model::types::RembgModel;
use crate::webui_assets::Assets;

pub async fn start(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Starting webui at http://{}", addr);

    let serve_assets = ServeEmbed::<Assets>::new();
    let app = Router::new()
        .route("/api/models", get(list_models))
        .route("/api/remove-bg", post(remove_background))
        .fallback_service(serve_assets);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn list_models() -> axum::Json<Vec<ModelInfo>> {
    let models: Vec<ModelInfo> = MODELS
        .iter()
        .map(|m| ModelInfo {
            name: m.name.to_string(),
            description: m.description.unwrap_or("").to_string(),
            downloaded: m.check_exists(),
        })
        .collect();
    axum::Json(models)
}

async fn remove_background(
    mut multipart: Multipart,
) -> Result<axum::Json<ApiResponse>, axum::Json<ApiResponse>> {
    let mut model_name: Option<String> = None;
    let mut image_bytes: Option<Bytes> = None;
    let mut extension = "png";

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();

        if name == "model" {
            if let Ok(text) = field.text().await {
                if !text.is_empty() {
                    model_name = Some(text);
                }
            }
        } else if name == "image" {
            let ct = field.content_type().unwrap_or("image/png");
            extension = match ct {
                "image/png" => "png",
                "image/jpeg" | "image/jpg" => "jpg",
                "image/webp" => "webp",
                "image/gif" => "gif",
                _ => "png",
            };
            if let Ok(bytes) = field.bytes().await {
                image_bytes = Some(bytes);
            }
        }
    }

    let bytes = match image_bytes {
        Some(b) => b,
        None => {
            return Err(axum::Json(ApiResponse::error("No file provided")));
        }
    };

    let model_name = match model_name {
        Some(m) => m,
        None => MODELS
            .iter()
            .find(|m| m.check_exists())
            .map(|m| m.name.to_string())
            .unwrap_or_else(|| "silueta".to_string()),
    };

    let model_info = match RembgModel::find_model(&model_name) {
        Some(m) => m,
        None => {
            return Err(axum::Json(ApiResponse::error(&format!(
                "Model {} not found",
                model_name
            ))));
        }
    };

    let model_path = match model_info.get_path() {
        Ok(p) => p,
        Err(e) => {
            return Err(axum::Json(ApiResponse::error(&format!(
                "Failed to get model path: {}",
                e
            ))));
        }
    };

    if !model_path.exists() {
        return Err(axum::Json(ApiResponse::error(
            "Model not found. Please download a model first using: nobg model pull <name>",
        )));
    }

    let temp_dir = std::env::temp_dir();
    let input_path = temp_dir.join(format!("nobg_input_{}.{}", std::process::id(), extension));
    let output_path = temp_dir.join(format!("nobg_output_{}.png", std::process::id()));

    if let Err(e) = std::fs::write(&input_path, &bytes) {
        return Err(axum::Json(ApiResponse::error(&format!(
            "Failed to write temp file: {}",
            e
        ))));
    }

    let _img = image::open(&input_path)
        .map_err(|e| ApiResponse::error(&format!("Failed to open image: {}", e)))?;
    let (tensor, original) =
        read_input_and_resize(&input_path.to_string_lossy(), model_info.resolution)
            .map_err(|e| ApiResponse::error(&format!("Failed to prepare input: {}", e)))?;
    let mask = run_inference(tensor, model_info)
        .map_err(|e| ApiResponse::error(&format!("Inference error: {}", e)))?;
    let result = save_output(
        mask,
        model_info.resolution,
        original,
        &output_path.to_string_lossy(),
    )
    .map_err(|e| ApiResponse::error(&format!("Failed to apply transparency: {}", e)));

    let _ = std::fs::remove_file(&input_path)
        .map_err(|e| ApiResponse::error(&format!("Failed to remove a file: {}", e)));

    match result {
        Ok(()) => {
            let output_bytes = match std::fs::read(&output_path) {
                Ok(b) => b,
                Err(e) => {
                    return Err(axum::Json(ApiResponse::error(&format!(
                        "Failed to read output: {}",
                        e
                    ))));
                }
            };

            let _ = std::fs::remove_file(&output_path);

            let base64_image = base64::engine::general_purpose::STANDARD.encode(&output_bytes);

            Ok(axum::Json(ApiResponse::success(base64_image)))
        }
        Err(e) => Err(axum::Json(e)),
    }
}

#[derive(serde::Serialize)]
struct ApiResponse {
    success: bool,
    image: Option<String>,
    error: Option<String>,
}

impl ApiResponse {
    fn success(image: String) -> Self {
        Self {
            success: true,
            image: Some(image),
            error: None,
        }
    }

    fn error(error: &str) -> Self {
        Self {
            success: false,
            image: None,
            error: Some(error.to_string()),
        }
    }
}

#[derive(serde::Serialize)]
struct ModelInfo {
    name: String,
    description: String,
    downloaded: bool,
}
