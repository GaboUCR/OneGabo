use tokio::io::{self, AsyncWriteExt};
use tokio::fs::{self, File};
use std::path::Path;

pub async fn create_folder (path:&str) -> io::Result<()> {
    let path = Path::new(path);

    // Attempt to create the directory
    if let Some(dir) = path.parent() {
        if let Err(e) = fs::create_dir_all(dir).await {
            eprintln!("Failed to create directory {}: {}", dir.display(), e);
            return Err(e);
        }
    }
    Ok(())
}

pub async fn delete_file_or_directory(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    
    if path.exists() {
        if path.is_dir() {
            // Recursively delete the directory and all its contents
            fs::remove_dir_all(path).await
        } else {
            // Delete a single file
            fs::remove_file(path).await
        }
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File or directory not found"))
    }
}

pub async fn save_bytes_to_file(data: &[u8], file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);

    create_folder(file_path).await;

    // Attempt to create the file
    let mut file = match File::create(path).await {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file {}: {}", path.display(), e);
            return Err(e);
        }
    };

    // Attempt to write data to the file
    if let Err(e) = file.write_all(data).await {
        eprintln!("Failed to write data to file {}: {}", path.display(), e);
        return Err(e);
    }

    Ok(())
}