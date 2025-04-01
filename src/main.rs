use clap::Parser;
use indicatif::ProgressBar;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Input directory to recursively read from
    input_dir: PathBuf,

    // Output directory to add files to in flat structure
    output_dir: PathBuf,
}

fn count_files(input_dir: &PathBuf) -> u64 {
    let mut count: u64 = 0;
    for entry in WalkDir::new(input_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            count += 1;
        }
    }
    count
}

fn copy_all(input_dir: PathBuf, output_dir: PathBuf, file_count: u64) -> Result<(), ()> {
    let bar = ProgressBar::new(file_count);
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
            thread::sleep(Duration::from_millis(100)); //TODO Remove this, just to test a progress bar. 
            bar.inc(1);
        }
    }
    bar.finish();
    Ok(())
}

fn main() {
    let args = Args::parse();
    let input_dir = args.input_dir;
    let output_dir = args.output_dir;
    if output_dir.exists() != true {
        panic!("Output directory does not exist!");
    }
    println!("Analyzing input directory...");
    let count = count_files(&input_dir);
    println!(
        "Found {} files. Beginning copy operations",
        count.to_string()
    );
    let _copy_all_result = copy_all(input_dir, output_dir, count);
}
