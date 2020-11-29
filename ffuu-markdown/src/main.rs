use async_std::io::prelude::{ReadExt, WriteExt};
use async_std::io::{stdin, stdout};
use pulldown_cmark::{html, Parser};

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
    let parser = Parser::new(&input);

    //
    // Render, Write
    //
    let mut output = String::new();
    html::push_html(&mut output, parser);

    //
    // Write
    //
    stdout().write_all(output.as_bytes()).await?;

    Ok(())
}
