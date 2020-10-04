use crate::file_helpers::{read_file, write_file};
use async_std::path::Path;

/// Add file to output directory
pub async fn add_file<P: AsRef<Path>, Q: AsRef<Path>>(
    output_dir: P,
    input_file_path: Q,
) -> anyhow::Result<()> {
    let contents = read_file(&input_file_path).await?;
    write_file(output_dir, input_file_path, contents).await?;

    Ok(())
}
