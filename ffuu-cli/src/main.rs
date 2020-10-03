use async_std::fs::{create_dir_all, remove_dir_all};
use async_std::io;

mod args;

use args::Arguments;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    prepare_output_dir(args.output_dir_path()).await?;

    Ok(())
}

/// Prepare output directory
///
/// Clear and create it.
///
async fn prepare_output_dir(path: &str) -> io::Result<()> {
    match remove_dir_all(path).await {
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        r => r,
    }?;
    create_dir_all(path).await
}
