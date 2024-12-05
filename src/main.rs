mod patch;
mod utils;

use crate::patch::{PatchSet, ZipPatcher};
use crate::utils::{get_or_pick_file, get_or_pick_folder, get_or_save_file};
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Target file to patch
    #[arg(short, long)]
    file: Option<String>,
    #[arg(short, long)]
    patches: Option<String>,
    /// Target destination to save
    #[arg(short, long)]
    output_file: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let input_file = get_or_pick_file(&args.file)
        .expect("Select a shader pack file or pass the path through the command line args!");
    let input_patches_folder = get_or_pick_folder(&args.patches)
        .expect("Select a folder containing patch files or pass through the command line args!");
    let output_file = get_or_save_file(&args.output_file)
        .expect("Select an output file or pass the path through the command line args!");

    let mut patcher = ZipPatcher::from(&input_file)?;
    let patches = PatchSet::from(&input_patches_folder)?;
    patcher.apply_patches(&patches)?;

    let file = std::fs::File::create(&output_file)?;
    let zip = patcher.save(file)?;
    zip.finish()?;
    println!("Output file written to: {:?}", &output_file);
    Ok(())
}
