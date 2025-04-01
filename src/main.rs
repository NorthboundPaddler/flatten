use clap::Parser;
use indicatif::ProgressBar;
use std::path::PathBuf;
use std::thread; // TODO Remove this, just for testing progress bar
use std::time::Duration; // TODO Remove this, just for testing progress bar
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
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Analyzing input directory...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    let mut count: u64 = 0;
    for entry in WalkDir::new(input_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            count += 1;
        }
    }

    thread::sleep(Duration::from_millis(10000)); //TODO Remove this, just to test a progress bar. 
    spinner.finish_with_message(format!("Found {} files", &count));
    count
}

fn copy_all(
    input_dir: PathBuf,
    output_dir: PathBuf,
    file_count: u64,
) -> Result<(), std::io::Error> {
    let bar = ProgressBar::new(file_count);
    for entry in WalkDir::new(input_dir).contents_first(true) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            //WARNING the filenames of photos are not unique outside directories,
            //so maybe append on the foldername?
            let output_path = &output_dir.join(entry.file_name());
            if output_path.exists() {
                // TODO Find a more graceful way to exit or queue up these errored files...
                return Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    format!("{} already exists!", output_path.to_str().unwrap()),
                ));
            }
            let _copy_result = std::fs::copy(entry.into_path(), output_path);
            thread::sleep(Duration::from_millis(100)); //TODO Remove this, just to test a progress bar. 
            bar.inc(1);
        }
    }
    bar.finish();
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let input_dir = args.input_dir;
    let output_dir = args.output_dir;
    if output_dir.exists() != true {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Output directory {} was not found!",
                output_dir.to_str().unwrap()
            ),
        ));
    }
    let count = count_files(&input_dir);
    let copy_all_result = copy_all(input_dir, output_dir, count);
    return copy_all_result;
}
