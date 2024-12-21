use std::{fs, io::{self, Write}, path::{Path, PathBuf}};
use reqwest::blocking::Client;
use zip::ZipWriter;
use zip::read::ZipArchive;
use std::io::Read;

const REMOTE_URL: &str = "https://github.com/writedan/divine-office";
const BRANCH_NAME: &str = "master";

pub fn clone_repo(path: &PathBuf) -> Result<(), String> {
    if fs::create_dir_all(path).is_err() {
        return Err(format!("Failed to create directory: {:?}", path));
    }

    let zip_url = format!("{}/archive/refs/heads/{}.zip", REMOTE_URL, BRANCH_NAME);

    println!("Downloading from {:?}", zip_url);
    let client = Client::new();
    let response = client.get(&zip_url).send()
        .map_err(|e| format!("Failed to fetch repository: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to download ZIP archive: {}", response.status()));
    }

    let zip_path = path.join("repo.zip");
    let mut file = fs::File::create(&zip_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    println!("Saving to {:?}", zip_path);
    io::copy(&mut response.bytes().map_err(|e| format!("Failed to read bytes: {}", e))?.as_ref(), &mut file)
        .map_err(|e| format!("Failed to write ZIP archive: {}", e))?;

    extract_zip(&zip_path, path)?;

    fs::remove_file(zip_path).map_err(|e| format!("Failed to delete ZIP file: {}", e))?;

    println!("Repository cloned successfully.");
    Ok(())
}

fn extract_zip(zip_path: &Path, destination: &Path) -> Result<(), String> {
    let file = fs::File::open(zip_path)
        .map_err(|e| format!("Failed to open ZIP file: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

    let mut folder_name = None;
    
    for i in 0..archive.len() {
        let file = archive.by_index(i).map_err(|e| format!("Failed to read file from ZIP: {}", e))?;
        
        if file.is_dir() {
            // Assuming there is only one folder, get the first one
            folder_name = Some(file.name().to_string());
            break;
        }
    }

    let folder_name = folder_name.ok_or_else(|| "No folder found in the ZIP archive.".to_string())?;
    println!("Identified base: {}", folder_name);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to read file from ZIP: {}", e))?;
        
        if file.name().starts_with(&folder_name) {
            let file_path = destination.join(file.name().strip_prefix(&folder_name).unwrap_or(file.name()));
            println!("\tExtracting to {:?}", file_path);

            if file.is_dir() {
                fs::create_dir_all(&file_path).map_err(|e| format!("Failed to create directory: {}", e))?;
            } else {
                let mut out_file = fs::File::create(&file_path)
                    .map_err(|e| format!("Failed to create file: {}", e))?;
                io::copy(&mut file, &mut out_file)
                    .map_err(|e| format!("Failed to extract file: {}", e))?;
            }
        }
    }

    Ok(())
}