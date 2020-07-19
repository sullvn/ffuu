use pulldown_cmark::{Parser, html};

const MARKDOWN: &str = "This is **bold**.";

fn main() {
    let md_parser = Parser::new(MARKDOWN);
    let mut html_output = String::new();

    html::push_html(&mut html_output, md_parser);
    println!("{}", html_output);
}
