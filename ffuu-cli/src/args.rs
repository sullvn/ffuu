use clap::{App, Arg, ArgMatches};

pub struct Arguments<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> Arguments<'a> {
    pub fn parse() -> Self {
        let matches = App::new("ffuu")
            .version("0.1")
            .author("Kevin Sullivan <kevin@sull.vn>")
            .about("Static site generator for people who hate static site generators")
            .arg(
                Arg::with_name("INPUT ROOT FILE")
                    .help("Input root HTML file to process, recursively following relative URIs")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("OUTPUT DIR")
                    .help("Output directory for processed files")
                    .required(true)
                    .index(2),
            )
            .get_matches();

        Arguments { matches }
    }

    pub fn input_file_path(self: &Self) -> &str {
        self.matches.value_of("INPUT ROOT FILE").unwrap()
    }

    pub fn output_dir_path(self: &Self) -> &str {
        self.matches.value_of("OUTPUT DIR").unwrap()
    }
}
