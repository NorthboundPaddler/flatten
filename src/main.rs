use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Input directory to recursively read from
    input_dir: PathBuf,

    // Output directory to add files to in flat structure
    output_dir: PathBuf,
}

fn main() {
    let args = Args::parse();
    for entry in WalkDir::new(args.input_dir).contents_first(true) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
        // TODO copy each entry that isn't a directory to the output_dir argument.
        // Check for a method of the DirEntry class to check if it's a directory.
        // Maybe create a progress bar output?
    }
}
