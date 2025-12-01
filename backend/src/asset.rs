use rust_embed::Embed;

#[derive(Embed)]
#[folder = "./"]
#[exclude = "src/*"]
#[exclude = ".gitignore"]
#[exclude = "Cargo.lock"]
#[exclude = "target/*"]
#[exclude = "Cargo.toml"]
#[exclude = ".git/*"]
/// This is used to embed all the liturgical files into the application itself so that no IO operations are ever used. This allows us to embed directly into WASM so that the frontend can run without a dedicated server.
pub struct Asset;
