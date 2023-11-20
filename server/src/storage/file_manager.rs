use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

pub async fn save_bytes_to_file(data: &[u8], file_path: &str) -> io::Result<()> {
    match File::create(file_path).await {
        Ok(mut file) => {
            match file.write_all(data).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    eprintln!("Failed to write to file: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            Err(e)
        }
    }
}