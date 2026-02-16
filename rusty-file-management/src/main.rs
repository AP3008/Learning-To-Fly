mod file_organizer;
use std::path::Path;

use crate::file_organizer::items_in_dir;

fn main() {
    println!("Hello, world!");
    let path_str: &str = "/Users/adamporbanderwalla/Desktop";
    let path = Path::new(path_str); 
    let res = match items_in_dir(path){
       Ok(res) => res,
       Err(e) => { 
            println!("Error reading path: {}", e);
            return; 
       }
    };
    println!("{:?}",res);
}


