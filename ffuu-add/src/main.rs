use async_std::io;
use std::env;

const OUTPUT_DIR_ENV_KEY: &str = "FFUU_OUTPUT_DIR";

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let output_dir = env::var(OUTPUT_DIR_ENV_KEY)?;
    println!("{}", output_dir);

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io::copy(&mut stdin, &mut stdout).await?;
    Ok(())
}
