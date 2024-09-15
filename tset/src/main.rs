use dirs;
use meow;
use std::io::{self, Write};
use std::path::PathBuf;

fn expand_home_path(path: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(path);
    if path_buf.starts_with("~") {
        if let Some(home_dir) = dirs::home_dir() {
            path_buf = home_dir.join(&path_buf.strip_prefix("~").unwrap());
        }
    }
    path_buf
}

fn main() {
    println!("please enter music file.");
    let mut str_ = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut str_).unwrap();
    let str_ = str_.trim();
    let sf = expand_home_path(str_);
    let s = meow::MeowMusics::new(sf.as_os_str().to_str().unwrap()).unwrap();
    let sf = sf.to_str().unwrap().split('/').last().unwrap();
    let s = s.play().unwrap();
    println!("---\nplay music: {}", sf);
    println!("time: {}", s.as_secs());

    std::thread::sleep(s);
}
