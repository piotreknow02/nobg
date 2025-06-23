use crate::model::types::RembgModel;

pub static MODELS: &[RembgModel] = &[
    RembgModel {
        name: "u2net",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net.onnx",
        description: Some("A pre-trained model for general use cases"),
    },
    RembgModel {
        name: "u2netp",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2netp.onnx",
        description: Some("A lightweight version of u2net model"),
    },
    RembgModel {
        name: "u2net_human_seg",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net_human_seg.onnx",
        description: Some("A pre-trained model for human segmentation"),
    },
    RembgModel {
        name: "u2net_cloth_seg",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net_cloth_seg.onnx",
        description: Some(
            "A pre-trained model for Cloths Parsing from human portrait. Here clothes are parsed into 3 category: Upper body, Lower body and Full body.",
        ),
    },
    RembgModel {
        name: "silueta",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/silueta.onnx",
        description: Some("Same as u2net but the size is reduced to 43Mb"),
    },
    RembgModel {
        name: "isnet-general-use",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/isnet-general-use.onnx",
        description: Some("A new pre-trained model for general use cases"),
    },
    RembgModel {
        name: "isnet-anime",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/isnet-anime.onnx",
        description: Some("A high-accuracy segmentation for anime character"),
    },
    RembgModel {
        name: "birefnet-general",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-general-epoch_244.onnx",
        description: Some("A pre-trained model for general use cases"),
    },
    RembgModel {
        name: "birefnet-general-lite",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-general-bb_swin_v1_tiny-epoch_232.onnx",
        description: Some("A light pre-trained model for general use cases"),
    },
    RembgModel {
        name: "birefnet-portrait",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-portrait-epoch_150.onnx",
        description: Some("A pre-trained model for human portraits"),
    },
    RembgModel {
        name: "birefnet-dis",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-DIS-epoch_590.onnx",
        description: Some("A pre-trained model for dichotomous image segmentation (DIS)"),
    },
    RembgModel {
        name: "birefnet-hrsod",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-HRSOD_DHU-epoch_115.onnx",
        description: Some(
            "A pre-trained model for high-resolution salient object detection (HRSOD)",
        ),
    },
    RembgModel {
        name: "birefnet-cod",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-COD-epoch_125.onnx",
        description: Some("A pre-trained model for concealed object detection (COD)"),
    },
    RembgModel {
        name: "birefnet-massive",
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-massive-TR_DIS5K_TR_TEs-epoch_420.onnx",
        description: Some("A pre-trained model with massive dataset"),
    },
    RembgModel {
        name: "ben2-base",
        remote_url: "https://huggingface.co/PramaLLC/BEN2/resolve/main/BEN2_Base.onnx",
        description: Some(
            "Introduces a novel approach to foreground segmentation through its innovative Confidence Guided Matting (CGM) pipeline",
        ),
    },
];
