use std::process::Command;
use rand::random_range; 
use std::path::{Path, PathBuf};
use std::fs::read_dir;
use chrono::{Local, NaiveTime, Timelike}; 

fn main() {
    let curr_time = Local::now().hour(); //gives us a u32 from 0 - 23
    let range = match curr_time {
        5..=7 => Range::Early_Morning,
        8..=11 => Range::Morning,
        12..=16 => Range::Day,
        17..=20 => Range::Evening,
        _ => Range::Night
    };
    
    let mut path: String = String::from("/Users/adamporbanderwalla/Desktop/Random Storage/Current-Wallpapers");
    match range {
        Range::Early_Morning => { path.push_str("/Early-Morning") },
        Range::Morning => { path.push_str("/Morning") },
        Range::Day => { path.push_str("/Day") },
        Range::Evening => { path.push_str("/Evening") },
        Range::Night => { path.push_str("/Night"); }
    }

    // Now we want to choose a random bg from one of those file paths
    
    let wp_path: &Path = Path::new(path.as_str());  
    let list_wp = items_in_dir(wp_path).unwrap(); 
    
    // Now generate a random number from 0 - list_wp.len()

    let num_files = list_wp.len(); 
    let index = rand::random_range(0..num_files);

    let wp_path: &str = list_wp[index].to_str().unwrap();

    // Now we have a random wallpaper, we just need to apply it. 
    let script = format!(
        "tell application \"System Events\"
            set picture of every desktop to \"{}\"
        end tell",
        wp_path
    );

    let _ = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .status(); 

    println!("Wallpaper changed!"); 
}


pub fn items_in_dir(path: &Path) -> Result<Vec<PathBuf>, String> {
    let iterator = match read_dir(path) {
        Ok(dir) => dir,
        Err(_e) => {
            println!("File path not found");
            return Err("Error".to_string());
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
        if file.path().is_file(){ 
            file_list.push(file.path());
        }
    }
    return Ok(file_list);
}

enum Range{
    Early_Morning, 
    Morning,
    Day,
    Evening,
    Night
}

