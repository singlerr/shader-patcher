use std::path::PathBuf;
use rfd::FileDialog;

pub fn get_or_pick_file(path_option: &Option<String>) -> Option<PathBuf>{
    if let Some(pth) = path_option{
        Some(PathBuf::from(pth))
    }else{
        FileDialog::new()
            .pick_file()
    }
}

pub fn get_or_pick_folder(path_option: &Option<String>) -> Option<PathBuf>{
    if let Some(pth) = path_option{
        Some(PathBuf::from(pth))
    }else{
        FileDialog::new()
            .pick_folder()
    }
}