mod patch;
mod utils;

use crate::utils::{get_or_pick_file, get_or_pick_folder};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Target file to patch
    #[arg(short, long)]
    input_file: Option<String>,
    input_patches_folder: Option<String>,
    /// Target destination to save
    #[arg(short, long)]
    output_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input_file = get_or_pick_file(&args.input_file)
        .expect("Select a shader pack file or pass the path through the command line args!");
    let input_patches_folder = get_or_pick_folder(&args.input_patches_folder)
        .expect("Select a folder containing patch files or pass through the command line args!");
    let output_file = get_or_pick_file(&args.output_file)
        .expect("Select an output file or pass the path through the command line args!");
}
