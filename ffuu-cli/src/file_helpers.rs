use anyhow::anyhow;
use async_std::fs::{read, write};
use async_std::path::Path;

/// Add file to output directory
pub async fn read_file<P: AsRef<Path>>(input_file_path: P) -> anyhow::Result<Vec<u8>> {
    let contents = read(input_file_path).await?;
    Ok(contents)
}

/// Add file to output directory
pub async fn write_file<P: AsRef<Path>, Q: AsRef<Path>, C: AsRef<[u8]>>(
    output_dir: P,
    input_file_path: Q,
    contents: C,
) -> anyhow::Result<()> {
    let input_file_name = input_file_path
        .as_ref()
        .file_name()
        .ok_or(anyhow!("No filename: {:?}", input_file_path.as_ref()))?;
    let output_path = output_dir.as_ref().join(input_file_name);

    write(output_path, contents).await?;

    Ok(())
}
