use async_std::path::Path;
use clap::{App, Arg, ArgMatches};

const OUTPUT_FILE_PATH_ARG: &str = "OUTPUT_FILE_PATH";

pub struct Arguments<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Arguments<'a> {
    pub fn parse() -> Self {
        let matches = App::new("ffuu-add")
            .version("0")
            .author("Kevin Sullivan <kevin@sull.vn>")
            .about("Add file to output directory")
            .arg(
                Arg::with_name(OUTPUT_FILE_PATH_ARG)
                    .help("Output file path. Relative to output directory")
                    .required(true)
                    .index(1),
            )
            .get_matches();

        Arguments { matches }
    }

    pub fn output_file_path(self: &Self) -> &Path {
        let path: &Path = self
            .matches
            .value_of(OUTPUT_FILE_PATH_ARG)
            .expect("Required argument")
            .as_ref();

        if path.is_absolute() {
            path.strip_prefix("/").expect("Absolute path")
        } else {
            path
        }
    }
}
