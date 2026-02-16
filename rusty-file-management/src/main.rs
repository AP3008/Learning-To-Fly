mod file_organizer;
use std::{fs::create_dir, path::Path};

use crate::file_organizer::{items_in_dir, organize_files};

fn main() {
    let path_str: &str = "/Users/adamporbanderwalla/Desktop";
    let path = Path::new(path_str); 
    let res = match items_in_dir(path){
       Ok(res) => res,
       Err(e) => { 
            println!("Error reading path: {}", e);
            return; 
       }
    };

    //println!("{:?}",res);
    let png = organize_files(res, "png");
    //println!("{:?}", png); 
}


