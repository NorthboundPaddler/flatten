use clap::{ArgAction, Parser};
use indicatif::ProgressBar;
use regex::Regex;
use std::path::PathBuf;
use std::time::Duration;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory to recursively read from
    input_dir: PathBuf,

    /// Output directory to add files to in flat structure
    output_dir: PathBuf,

    /// Optional Regular Expression (regex) to use as a filter
    #[arg(required = false)]
    #[arg(short = 'f')]
    #[arg(long = "filter")]
    #[arg(help = "Regular Expression to filter files by")]
    filter_expression: Option<String>,

    /// Option to overwrite duplicate output filenames
    #[arg(required = false)]
    #[arg(short = 'd')]
    #[arg(long = "duplicates")]
    #[arg(action=ArgAction::SetTrue)]
    duplicates: Option<bool>,

    /// Option to "skip" the copy action and just print counts
    #[arg(required = false)]
    #[arg(long = "dry")]
    #[arg(action=ArgAction::SetTrue)]
    dry: Option<bool>,
}

fn matches_regex(entry: &walkdir::DirEntry, regex: &Regex) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| regex.is_match(name))
        .unwrap_or(false)
}

fn count_files(input_dir: &PathBuf, regex: Option<&Regex>) -> u64 {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Analyzing input directory...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    let count = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            if let Some(regex) = regex {
                matches_regex(entry, regex)
            } else {
                true
            }
        })
        .count();
    spinner.finish_and_clear();
    let count_rv = count as u64;
    count_rv
}

fn copy_all(
    input_dir: PathBuf,
    output_dir: PathBuf,
    file_count: u64,
    regex: Option<&Regex>,
    duplicates: Option<&bool>,
    dry: Option<&bool>,
) -> Result<(), std::io::Error> {
    let mut copy_count: u64 = 0;
    let mut dupe_count: u64 = 0;
    let bar = ProgressBar::new(file_count);
    let accept_dupes = duplicates.unwrap_or(&false);
    for entry in WalkDir::new(input_dir).contents_first(true) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            // Filter files based on regex if provided
            if let Some(regex) = regex {
                if !matches_regex(&entry, regex) {
                    continue;
                }
            }
            let output_path = &output_dir.join(entry.file_name());
            if output_path.exists() {
                dupe_count += 1;
                if *accept_dupes {
                    continue;
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::AlreadyExists,
                        format!("{} already exists!", output_path.to_str().unwrap()),
                    ));
                }
            }
            if !dry.unwrap() {
                let _copy_result = std::fs::copy(entry.into_path(), output_path);
            }
            copy_count += 1;
            bar.inc(1);
        }
    }
    bar.finish();
    // WARNING: This doesn't properly count duplicates for the --dry option
    println!(
        "Total Count: {}\nCopied: {}\nDuplicates: {}",
        file_count, copy_count, dupe_count,
    );
    Ok(())
}

fn run() -> Result<(), std::io::Error> {
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
    // Parse the optional filter expression into a Regex object
    let regex = match args.filter_expression {
        Some(expr) => Some(Regex::new(&expr).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid regex: {}", e),
            )
        })?),
        None => None,
    };

    let count = count_files(&input_dir, regex.as_ref());
    let copy_all_result = copy_all(
        input_dir,
        output_dir,
        count,
        regex.as_ref(),
        args.duplicates.as_ref(),
        args.dry.as_ref(),
    );
    return copy_all_result;
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1)
    }
}
