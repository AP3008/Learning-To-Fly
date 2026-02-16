use std::fs::{create_dir, read_dir, rename};
use std::io::Error;
use std::path::{Path, PathBuf};

// This function simply gets us a list of paths in a directory
pub fn items_in_dir(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let iterator = match read_dir(path) {
        Ok(dir) => dir,
        Err(e) => {
            println!("File path not found");
            return Err(e);
        }
    };

    let mut file_list: Vec<PathBuf> = Vec::new();

    for file in iterator {
        //We need to match each of the files to DirEntry from Result
        let file = match file {
            Ok(f) => f,
            Err(_e) => {
                continue;
            }
        };
        file_list.push(file.path());
    }
    return Ok(file_list);
}

// Based of a list of paths we organize them into folders based of file extension
pub fn group_files(paths: Vec<PathBuf>, extension: &str) -> Vec<PathBuf> {
    let mut grouped_list: Vec<PathBuf> = Vec::new();

    for path in paths {
        match get_extension(&path) {
            Some(ext) => {
                if extension == ext {
                    grouped_list.push(path.clone());
                }
            }
            None => (),
        };
    }
    return grouped_list;
}

// Grabbing the extension from the path
fn get_extension(file_path: &Path) -> Option<&str> {
    match file_path.extension() {
        Some(os_str) => os_str.to_str(),
        None => None,
    }
}

//Function that creates a directory based on the extension name
//Making this specific to me so I am creating the directories inside of Random Storage on my PC
fn create_extension_dir(extension: &str, location: &Path) -> () {
    match create_dir(location.join(extension.to_uppercase())) {
        Ok(_d) => {
            println!("Created {extension} dir")
        }
        Err(e) => {
            println!("Error: {e}")
        }
    }
}

//Organizes files into respective folders
pub fn organize(collection: Vec<PathBuf>, extension: &str, location: &Path) -> () {
    //Check if a folder exists
    if !(location.exists()) {
        create_extension_dir(extension, &location);
    }
    for file in collection {
        match rename(&file, &location) {
            Ok(_f) => {
                println!("file moved!")
            }
            Err(e) => {
                println!("Did not move: {e}")
            }
        };
    }
}
