use std::{env, process::Command};
use std::time::{Instant};
use jwalk::{WalkDir};


fn main() {
    let now = Instant::now();
     let mut vec = Vec::<String>::new();
     for arg in env::args().skip(1) {
             for entry in WalkDir::new(arg)
                 .follow_links(true)
                 .into_iter()
                 .filter_map(|e| e.ok())
                 .filter(|e| !e.file_type().is_dir())
                  {
                     let mut cmd = Command::new("sh");
                     cmd.arg("-c").arg(format!("md5sum \"{}\"", entry.path().display()));
                     let output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
                     vec.push(output);
                 }            
     }
     let mut cmd = Command::new("sh");
     cmd.arg("-c").arg(format!("echo \"{}\" | md5sum", vec.concat()));
     let output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
     println!("{}", now.elapsed().subsec_millis());
     print!("{}\n", output);
 }