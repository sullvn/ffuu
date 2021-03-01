use async_std::io::prelude::{ReadExt, WriteExt};
use async_std::io::{stdin, stdout};
use html_parse::{format_html, parse_html};

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
    let result = parse_html(&input)?;

    //
    // Render
    //
    let output = format_html(result);

    //
    // Write
    //
    stdout().write_all(output.as_bytes()).await?;

    Ok(())
}
