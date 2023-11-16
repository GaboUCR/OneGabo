use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

pub async fn save_bytes_to_file(data: &[u8], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path).await?;
    file.write_all(data).await?;
    Ok(())
}