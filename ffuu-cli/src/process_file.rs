use anyhow::anyhow;
use async_std::fs::{read, write};
use async_std::path::Path;

/// Process file, reading, processing, and writing out
pub async fn process_file<P: AsRef<Path>>(output_dir: P, input_file_path: P) -> anyhow::Result<()> {
    let input_file_name = input_file_path
        .as_ref()
        .file_name()
        .ok_or(anyhow!("No filename: {:?}", input_file_path.as_ref()))?;
    let output_path = output_dir.as_ref().join(input_file_name);

    let contents = read(input_file_path).await?;
    write(output_path, contents).await?;

    Ok(())
}
