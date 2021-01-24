use crate::add_file::add_file;
use crate::file_helpers::{read_file, write_file};
use anyhow::anyhow;
use async_std::path::Path;
use html_parse::{parse_all_parts, HTMLPart, URI_HTML_ATTRIBUTES};
use std::str;
use url::Url;

/// Add file to output directory
pub async fn add_html_file<P: AsRef<Path>, Q: AsRef<Path>>(
    output_dir: P,
    input_file_path: Q,
) -> anyhow::Result<()> {
    let input_dir = input_file_path
        .as_ref()
        .parent()
        .ok_or(anyhow!("Can't get parent of directory"))?;
    let contents = read_file(&input_file_path).await?;
    let html = str::from_utf8(&contents)?;
    let relative_paths = find_relative_paths(html)?;
    for rp in relative_paths {
        add_file(&output_dir, input_dir.join(rp)).await?;
    }

    write_file(output_dir, input_file_path, contents).await?;

    Ok(())
}

fn find_relative_paths(html: &str) -> anyhow::Result<Vec<&str>> {
    let mut rps = Vec::new();
    let parts = match parse_all_parts(html) {
        Ok((_, parts)) => Ok(parts),
        Err(err) => Err(anyhow!("Can't parse HTML: {}", err)),
    }?;

    for p in parts {
        if let HTMLPart::Tag(tag) = p {
            for attr in tag.attributes {
                if let (name, Some(value)) = attr {
                    let is_uri_attribute = URI_HTML_ATTRIBUTES.contains(name);
                    let is_absolute_url = is_uri_attribute && Url::parse(value).is_ok();
                    let is_relative_path = !is_absolute_url && Path::new(value).is_relative();

                    if is_uri_attribute && !is_absolute_url && is_relative_path {
                        rps.push(value);
                    }
                }
            }
        }
    }

    Ok(rps)
}
