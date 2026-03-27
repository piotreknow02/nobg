use crate::model::types::RembgModel;

pub const MODELS: &[RembgModel] = &[
    RembgModel {
        name: "u2net",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net.onnx",
        description: Some("A pre-trained model for general use cases"),
        checksum: Some("8d10d2f3bb75ae3b6d527c77944fc5e7dcd94b29809d47a739a7a728a912b491"),
    },
    RembgModel {
        name: "u2netp",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2netp.onnx",
        description: Some("A lightweight version of u2net model"),
        checksum: Some("988311b63e23adc2596e7f5d4d1c28cd301b1641b2a211c3168f1896f96ee8da"),
    },
    RembgModel {
        name: "u2net_human_seg",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net_human_seg.onnx",
        description: Some("A pre-trained model for human segmentation"),
        checksum: Some("01eb6a29a5c4d8edb30b56adad9bb3a2a0535338e480724a213e0acfd2d1c73c"),
    },
    RembgModel {
        name: "u2net_cloth_seg",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2net_cloth_seg.onnx",
        description: Some(
            "A pre-trained model for Cloths Parsing from human portrait. Here clothes are parsed into 3 category: Upper body, Lower body and Full body.",
        ),
        checksum: Some("6d2cbc27bfbdc989e1fd325656d65902ecc6a3ccbe94b2d3655ec114efcb128e"),
    },
    RembgModel {
        name: "silueta",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/silueta.onnx",
        description: Some("Same as u2net but the size is reduced to 43Mb"),
        checksum: Some("75da6c8d2f8096ec743d071951be73b4a8bc7b3e51d9a6625d63644f90ffeedb"),
    },
    RembgModel {
        name: "isnet-general-use",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/isnet-general-use.onnx",
        description: Some("A new pre-trained model for general use cases"),
        checksum: Some("60920e99c45464f2ba57bee2ad08c919a52bbf852739e96947fbb4358c0d964a"),
    },
    RembgModel {
        name: "isnet-anime",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/isnet-anime.onnx",
        description: Some("A high-accuracy segmentation for anime character"),
        checksum: Some("f15622d853e8260172812b657053460e20806f04b9e05147d49af7bed31a6e99"),
    },
    RembgModel {
        name: "birefnet-general",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-general-epoch_244.onnx",
        description: Some("A pre-trained model for general use cases"),
        checksum: Some("58f621f00f5d756097615970a88a791584600dcf7c45b18a0a6267535a1ebd3c"),
    },
    RembgModel {
        name: "birefnet-general-lite",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-general-bb_swin_v1_tiny-epoch_232.onnx",
        description: Some("A light pre-trained model for general use cases"),
        checksum: Some("5600024376f572a557870a5eb0afb1e5961636bef4e1e22132025467d0f03333"),
    },
    RembgModel {
        name: "birefnet-portrait",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-portrait-epoch_150.onnx",
        description: Some("A pre-trained model for human portraits"),
        checksum: Some("1ba1c8ff5a7bbfadc8d8d13fb11d7be793f91f23d9d466549e37a854f6668f99"),
    },
    RembgModel {
        name: "birefnet-dis",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-DIS-epoch_590.onnx",
        description: Some("A pre-trained model for dichotomous image segmentation (DIS)"),
        checksum: Some("6470117bac6f8d82a3f62921056f52d0f5c4d36d1d832096331d5ea38a03acb5"),
    },
    RembgModel {
        name: "birefnet-hrsod",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-HRSOD_DHU-epoch_115.onnx",
        description: Some(
            "A pre-trained model for high-resolution salient object detection (HRSOD)",
        ),
        checksum: Some("4f5837663194fb88f603b76782eae05a3c29f5749872ca1bfb636bd26e7f6bfc"),
    },
    RembgModel {
        name: "birefnet-cod",
        resolution: (320, 320),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-COD-epoch_125.onnx",
        description: Some("A pre-trained model for concealed object detection (COD)"),
        checksum: Some("91ec48f566db475cf6e4caa7e9cd997f352edfcc372372f437e2fbefc1557b13"),
    },
    RembgModel {
        name: "birefnet-massive",
        resolution: (1000, 1000),
        remote_url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/BiRefNet-massive-TR_DIS5K_TR_TEs-epoch_420.onnx",
        description: Some("A pre-trained model with massive dataset"),
        checksum: Some("a94814cac438a31f95287811882628644a04b22d313ef3071d2ba904b5f627b8"),
    },
    RembgModel {
        name: "ben2-base",
        resolution: (320, 320),
        remote_url: "https://huggingface.co/PramaLLC/BEN2/resolve/main/BEN2_Base.onnx",
        description: Some(
            "Introduces a novel approach to foreground segmentation through its innovative Confidence Guided Matting (CGM) pipeline",
        ),
        checksum: Some("22cea62108ff53b7ccc20f7a008bf30494228d84b1687f29ecbe76936a998101"),
    },
    RembgModel {
        name: "trendyol",
        resolution: (1800, 1200),
        remote_url: "https://huggingface.co/Trendyol/background-removal/resolve/main/model.onnx",
        description: Some(
            "Model mainly for human segmentation trained on fashion imagery by turkish company Trendyol",
        ),
        checksum: Some("b9ef43bab5c1a5538916d31066d5ae843a2200d1fd83ed5bac320643076332d0"),
    },
];
