use std::io::Error;
use std::fs::{DirEntry, ReadDir, create_dir, read_dir, rename};
use std::path::{Path, PathBuf}; 


pub fn items_in_dir(path:&Path) -> Result<Vec<PathBuf>, Error>{
    let iterator = match read_dir(path){
        Ok(dir) => dir, 
        Err(e) => {
            println!("File path not found");
            return Err(e); 
        }
    };
    
    let mut file_list: Vec<PathBuf> = Vec::new(); 

    for file in iterator{
        //We need to match each of the files to DirEntry from Result
        let file = match file {
            Ok(f) => f,
            Err(e) => { continue; }
        };
        file_list.push(file.path());
    }
    return Ok(file_list);  
}
