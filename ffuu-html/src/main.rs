use anyhow::anyhow;
use async_std::io::prelude::{ReadExt, WriteExt};
use async_std::io::{stdin, stdout};
use html_parse::{format_html, parse_embeds, parse_html, HTMLEmbed, HTMLPart, HTMLPartOrEmbed};
use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    //
    // Read
    //
    let mut input = String::new();
    stdin().read_to_string(&mut input).await?;

    //
    // Parse HTML
    //
    let input_parts = parse_html(&input)?;
    let input_parts_count = input_parts.len();

    //
    // Parse embeds
    //
    let with_embeds = parse_embeds(input_parts);

    //
    // Execute embeds
    //
    let mut result_parts: Vec<HTMLPart> = Vec::with_capacity(input_parts_count);
    for part_or_embed in with_embeds {
        match part_or_embed {
            HTMLPartOrEmbed::Part(part) => result_parts.push(part),
            HTMLPartOrEmbed::Embed(embed) => {
                let result = exec_embed(&embed);
                if let Ok(output) = result {
                    result_parts.push(HTMLPart::Text(output.into()));
                }
            }
        }
    }

    //
    // Render
    //
    let output = format_html(&result_parts);

    //
    // Write
    //
    stdout().write_all(output.as_bytes()).await?;

    Ok(())
}

fn exec_embed(request: &HTMLEmbed) -> anyhow::Result<String> {
    let HTMLEmbed { command, input } = request;
    let stdin = if input.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    };

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(stdin)
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(input_text) = input {
        child
            .stdin
            .as_mut()
            .ok_or(anyhow!("Can't borrow stdin as mutable"))?
            .write_all(input_text.as_bytes())?;
    }
    let output = child.wait_with_output()?;
    let text = str::from_utf8(&output.stdout)?;
    let trimmed = text.trim();
    let string = trimmed.to_owned();

    Ok(string)
}
