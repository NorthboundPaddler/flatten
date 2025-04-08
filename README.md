# Flatten

A simple CLI tool to recursively pull out a filtered set of files and copy them to an output directory. Written in Rust.

## Documentation

To get started quickly, you can run:

```sh
flatten ~/test_inputs ~/test_outputs 
```

Here's a full list of options:

```
Usage: extract-google-photos [OPTIONS] <INPUT_DIR> <OUTPUT_DIR>

Arguments:
  <INPUT_DIR>   Input directory to recursively read from
  <OUTPUT_DIR>  Output directory to add files to in flat structure

Options:
  -f, --filter <FILTER_EXPRESSION>  Regular Expression to filter files by
  -d, --duplicates                  Option to overwrite duplicate output filenames
      --dry                         Option to "skip" the copy action and just print counts
  -h, --help                        Print help
  -V, --version                     Print version
```

## TODO

- [x] List all files, recursively
- [X] Copy all files to single output directory
- [X] Show progress bar
- [X] Add a "filter" optional argument
- [x] Rename and open source this repo on github as "flatten"
- [ ] Functionalize the "walkdir" used twice in the same main.rs...
