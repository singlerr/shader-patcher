use std::{
    collections::{HashMap, HashSet},
    fs,
    fs::File,
    io::{BufReader, Error, ErrorKind, Read, Result, Write},
    path::PathBuf,
};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};

pub struct ZipPatcher {
    entries: HashMap<PathBuf, Vec<u8>>,
    new_entries: HashMap<PathBuf, Vec<u8>>,
}

pub struct Patch {
    path: PathBuf,
    contents: Vec<u8>,
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
        for dir in walkdir::WalkDir::new(patch_folder) {
            let path = dir?.path().to_path_buf();
            let patch_path = path.strip_prefix(patch_folder).unwrap().to_path_buf();

            if !Self::is_patch_file(&path) {
                println!("Skipping non-patch(.patch) file {:?}", &patch_path);
                continue;
            }

            let file = File::open(&path)?;
            let mut reader = BufReader::new(file);
            let mut patch_contents = Vec::new();
            reader.read_to_end(&mut patch_contents)?;

            println!("Reading patch file {:?}", &patch_path);

            patches.push(Patch {
                path: patch_path,
                contents: patch_contents,
            });
        }

        Ok(PatchSet { patches })
    }

    fn is_patch_file(path: &PathBuf) -> bool {
        let ext = path.extension();
        match ext {
            Some(ext) => ext == "patch",
            None => false,
        }
    }
}

impl ZipPatcher {
    pub fn from(input_path: &PathBuf) -> Result<Self> {
        let file = File::open(input_path)?;
        let reader = BufReader::new(file);
        let mut zip = ZipArchive::new(reader)?;
        let mut entries: HashMap<PathBuf, Vec<u8>> = HashMap::new();
        let zip_file_name = input_path.file_name().unwrap();
        for i in 0..zip.len() {
            let Ok(mut entry) = zip.by_index(i) else {
                println!(
                    "Ignored file indexed by {} in {:?} due to errors while reading",
                    i, zip_file_name
                );
                continue;
            };
            let Some(entry_path) = entry.enclosed_name() else {
                println!("Ignored file with name {} in {:?} due to errors while getting its enclosed name", &entry.name(), zip_file_name);
                continue;
            };

            println!("Reading zip content {:?}", &entry_path);

            let mut data = Vec::new();
            entry.read_to_end(&mut data)?;
            &entries.insert(entry_path, data);
        }

        Ok(ZipPatcher {
            entries,
            new_entries: HashMap::new(),
        })
    }

    pub fn apply_patches(&mut self, patch_set: &PatchSet) -> Result<()> {
        let mut deleted_files: HashSet<PathBuf> = HashSet::new();
        for patch in &patch_set.patches {
            let path = patch.path.clone().with_extension("");
            if self.entries.contains_key(&path) {
                let old_text = self.entries.get(&path).unwrap();
                let mut bsdiff_patch = patch.contents.as_slice();
                let mut new_text = Vec::new();
                match bsdiff::patch(old_text.as_slice(), &mut bsdiff_patch, &mut new_text) {
                    Ok(_) => {
                        self.new_entries.insert(path.clone(), new_text);
                        &deleted_files.insert(path.clone());
                        println!("Patch applied {:?} > {:?}", &patch.path, &path);
                    }
                    Err(e) => {
                        println!(
                            "Failed to apply patch {:?}. Is it generated by bsdiff-rs?",
                            &patch.path
                        );
                    }
                }
            }
        }

        for (key, value) in self.entries.iter() {
            if *&deleted_files.contains(key) {
                continue;
            }
            if self.new_entries.contains_key(key) {
                continue;
            }
            self.new_entries.insert(key.clone(), value.clone());
        }
        Ok(())
    }

    pub fn save(&self, file: File) -> Result<ZipWriter<File>> {
        let mut zip = ZipWriter::new(file);
        let opt = SimpleFileOptions::default();
        for (path, data) in self.new_entries.iter() {
            &zip.start_file_from_path(path, opt);
            &zip.write_all(data);

            println!("Writing {:?}", path);
        }
        Ok(zip)
    }
}
