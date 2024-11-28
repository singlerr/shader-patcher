use diff_match_patch_rs::DiffMatchPatch;
use std::io::read_to_string;
use std::{
    fs::{File, Path, ReadDir},
    io::{BufReader, Error, ErrorKind, Result},
    path::PathBuf,
};
use zip::ZipArchive;

pub struct ZipPatcher {
    zip_file: ZipArchive<BufReader<File>>,
}

pub struct Patch {
    name: String,
    contents: String,
}

pub struct PatchSet {
    patches: Vec<Patch>,
}

impl PatchSet {
    pub fn from(patch_folder: &PathBuf) -> Result<Self> {
        if !patch_folder.is_dir() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "target path is not dir.",
            ));
        }

        let mut patches: Vec<Patch> = Vec::new();
        for dir in patch_folder.read_dir()? {
            let path = dir?.path();
            if !Self::is_patch_file(&path) {
                continue;
            }

            let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
            let patch_contents = read_to_string(&path)?;
            patches.push(Patch {
                name: file_name,
                contents: patch_contents,
            });
        }

        Ok(PatchSet { patches })
    }

    fn is_patch_file(path: &PathBuf) -> bool {
        let ext = path.extension();
        match ext {
            Some(ext) => ext == "patch",
            None() => false,
        }
    }
}

impl ZipPatcher {
    pub fn from(input_path: &PathBuf) -> Result<Self> {
        let file = File::open(input_path)?;
        let reader = BufReader::new(file);
        let zip = ZipArchive::new(reader);

        Ok(ZipPatcher { zip_file: zip? })
    }

    pub fn apply_patches(patch_set: &PatchSet) -> Result<()> {
        todo!()
    }

    pub fn save(file_path: &PathBuf) -> Result<()> {
        todo!()
    }
}
