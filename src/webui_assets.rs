use rust_embed::RustEmbed;

#[derive(RustEmbed, Clone)]
#[folder = "webui/"]
pub struct Assets;
