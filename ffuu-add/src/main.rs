use async_std::io;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io::copy(&mut stdin, &mut stdout).await?;
    Ok(())
}
