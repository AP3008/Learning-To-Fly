mod file_organizer;

use std::path::{Path, PathBuf};
use crate::file_organizer::{items_in_dir, group_files, organize};

fn main() {
    let area_to_clean: &Path = Path::new("/Users/adamporbanderwalla/Desktop"); 
    let destination: &Path = Path::new("/Users/adamporbanderwalla/Desktop/Random Storage");
    
    let extensions:[&str; 14] = ["webp", "c", "dSYM", "pdf", "mov","zip", "mp4", "m4a", "png", "jpg", "jpeg", "docx", "txt", "gif"];  
    
    let items_dir = match items_in_dir(area_to_clean){
        Ok(dir) => dir, 
        Err(_e) => Vec::<PathBuf>::new(),  
    };
    let random_items_dir = match items_in_dir(destination){
        Ok(dir) => dir,
        Err(_e) => Vec::<PathBuf>::new()
    };
    for ext in extensions{
        let group: Vec<PathBuf> = group_files(items_dir.clone(), ext);
        let random_group: Vec<PathBuf> = group_files(random_items_dir.clone(), ext); 
        organize(group, ext, destination);
        organize(random_group, ext, destination); 
    }
}
