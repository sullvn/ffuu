mod args;

use args::Arguments;
use async_std::fs::{create_dir_all, File};
use async_std::io;
use async_std::path::{Path, PathBuf};
use io::prelude::WriteExt;
use std::env;

const OUTPUT_DIR_ENV_KEY: &str = "FFUU_OUTPUT_DIR";

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let output_dir_env = env::var(OUTPUT_DIR_ENV_KEY)?;

    let output_dir: &Path = output_dir_env.as_ref();
    let output_file_path_relative = args.output_file_path();
    let output_file_path: PathBuf = [output_dir, output_file_path_relative].iter().collect();
    let website_file_path: PathBuf = ["/".as_ref(), output_file_path_relative].iter().collect();
    let website_file_path_str: &str = website_file_path
        .to_str()
        .ok_or(anyhow::anyhow!("Bad filename"))?;
    let output_file_dir: PathBuf = [
        output_dir,
        output_file_path_relative
            .parent()
            .unwrap_or_else(|| Path::new("\\")),
    ]
    .iter()
    .collect();

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    create_dir_all(output_file_dir).await?;
    let mut output_file = File::create(output_file_path).await?;
    io::copy(&mut stdin, &mut output_file).await?;
    stdout.write_all(website_file_path_str.as_bytes()).await?;

    Ok(())
}
