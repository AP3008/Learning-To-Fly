mod file_organizer;
use std::{fs::create_dir, path::Path};

use crate::file_organizer::{items_in_dir, group_files};

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
    let png = group_files(res, "png");
    //println!("{:?}", png); 
}


