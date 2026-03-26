use crate::model::types::RembgModel;

pub static MODELS: &[RembgModel] = &[
    RembgModel {
        name: "u2net",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net.onnx",
        description: Some("A pre-trained model for general use cases"),
        checksum: None,
    },
    RembgModel {
        name: "u2netp",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2netp.onnx",
        description: Some("A lightweight version of u2net model"),
        checksum: None,
    },
    RembgModel {
        name: "u2net_human_seg",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net_human_seg.onnx",
        description: Some("A pre-trained model for human segmentation"),
        checksum: None,
    },
    RembgModel {
        name: "u2net_cloth_seg",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net_cloth_seg.onnx",
        description: Some(
            "A pre-trained model for Cloths Parsing from human portrait. Here clothes are parsed into 3 category: Upper body, Lower body and Full body.",
        ),
        checksum: None,
    },
    RembgModel {
        name: "silueta",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/silueta.onnx",
        description: Some("Same as u2net but the size is reduced to 43Mb"),
        checksum: None,
    },
    RembgModel {
        name: "isnet-general-use",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/isnet-general-use.onnx",
        description: Some("A new pre-trained model for general use cases"),
        checksum: None,
    },
    RembgModel {
        name: "isnet-anime",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/isnet-anime.onnx",
        description: Some("A high-accuracy segmentation for anime character"),
        checksum: None,
    },
    RembgModel {
        name: "birefnet-general",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-general-epoch_244.onnx",
        description: Some("A pre-trained model for general use cases"),
        checksum: None,
    },
    RembgModel {
        name: "birefnet-general-lite",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-general-bb_swin_v1_tiny-epoch_232.onnx",
        description: Some("A light pre-trained model for general use cases"),
        checksum: None,
    },
    RembgModel {
        name: "birefnet-portrait",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-portrait-epoch_150.onnx",
        description: Some("A pre-trained model for human portraits"),
        checksum: None,
    },
    RembgModel {
        name: "birefnet-dis",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-DIS-epoch_590.onnx",
        description: Some("A pre-trained model for dichotomous image segmentation (DIS)"),
        checksum: None,
    },
    RembgModel {
        name: "birefnet-hrsod",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-HRSOD_DHU-epoch_115.onnx",
        description: Some(
            "A pre-trained model for high-resolution salient object detection (HRSOD)",
        ),
        checksum: None,
    },
    RembgModel {
        name: "birefnet-cod",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-COD-epoch_125.onnx",
        description: Some("A pre-trained model for concealed object detection (COD)"),
        checksum: None,
    },
    RembgModel {
        name: "birefnet-massive",
        resolution: (1000, 1000),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-massive-TR_DIS5K_TR_TEs-epoch_420.onnx",
        description: Some("A pre-trained model with massive dataset"),
        checksum: None,
    },
    RembgModel {
        name: "ben2-base",
        resolution: (320, 320),
        remote_url: "https://huggingface.co/PramaLLC/BEN2/resolve/main/BEN2_Base.onnx",
        description: Some(
            "Introduces a novel approach to foreground segmentation through its innovative Confidence Guided Matting (CGM) pipeline",
        ),
        checksum: None,
    },
    RembgModel {
        name: "trendyol",
        resolution: (1800, 1200),
        remote_url: "https://huggingface.co/Trendyol/background-removal/resolve/main/model.onnx",
        description: Some(
            "Model mainly for human segmentation trained on fashion imagery by turkish company Trendyol",
        ),
        checksum: None,
    },
];
