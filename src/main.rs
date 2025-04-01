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

fn count_files(input_dir: &PathBuf) -> u128 {
    let mut count: u128 = 0;
    for entry in WalkDir::new(input_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            count += 1;
        }
    }
    count
}

fn copy_all(input_dir: PathBuf, output_dir: PathBuf) -> Result<(), ()> {
    for entry in WalkDir::new(input_dir).contents_first(true) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            //WARNING the filenames of photos are not unique outside directories,
            //so maybe append on the foldername?
            let output_path = &output_dir.join(entry.file_name());
            if output_path.exists() {
                // TODO Find a more graceful way to exit or queue up these errored files...
                panic!("The file {} already exists!", output_path.to_str().unwrap());
            }
            let _copy_result = std::fs::copy(entry.into_path(), output_path);
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    let input_dir = args.input_dir;
    let output_dir = args.output_dir;
    println!("Analyzing input directory...");
    let count = count_files(&input_dir);
    println!(
        "Found {} files. Beginning copy operations",
        count.to_string()
    );
    let _copy_all_result = copy_all(input_dir, output_dir);
}
