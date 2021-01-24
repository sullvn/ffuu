use async_std::io::prelude::{ReadExt, WriteExt};
use async_std::io::{stdin, stdout};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    //
    // Read
    //
    let mut input = String::new();
    stdin().read_to_string(&mut input).await?;

    //
    // Parse
    //
    // NOOP

    //
    // Render
    //
    let mut output = String::new();
    output.push_str(&input);

    //
    // Write
    //
    stdout().write_all(output.as_bytes()).await?;

    Ok(())
}
