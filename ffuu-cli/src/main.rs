mod args;

use args::Arguments;

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args.input_file_path());
    println!("{:?}", args.output_dir_path());
}
