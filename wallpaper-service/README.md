# Rust Wallpaper Service

A way for me to programmatically change my wallpaper depending on the time of day. Initially I was planning on doing this using the wallpaper crate but I ran into the issue of it not working accross all my spaces. Currently it doesn't work well across all spaces, but it does change the current space when you run the program. Currently it is super tailored to my personal needs so cloning this for yourself doesn't really work but you could fork my repo and change this specific project. 

## How I am using it

To get it working system wide, I used 
```bash
cargo build --release .
```
Which then installed it on my system, but because I wanted this running as a service I created a .plist and put it in my LaunchAgents, so now on an interval the program changes the background of my current space, I also made it so it runs on launch. 
