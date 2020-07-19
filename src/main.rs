use std::env;
use std::str;
use async_std::fs::{read, write};
use pulldown_cmark::{Parser, html};

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).unwrap();

    let input_bytes = read(input_path).await?;
    let input_text: &str = str::from_utf8(&input_bytes).unwrap();

    let md_parser = Parser::new(input_text);
    let mut html_output = String::new();

    html::push_html(&mut html_output, md_parser);
    println!("{}", html_output);

    Ok(())
}
