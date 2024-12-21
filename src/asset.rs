use rust_embed::Embed;

#[derive(Embed)]
#[folder = "./"]
#[exclude = "src/*"]
#[exclude = ".gitignore"]
#[exclude = "Cargo.lock"]
#[exclude = "target/*"]
#[exclude = "Cargo.toml"]
#[exclude = ".git/*"]
pub struct Asset;