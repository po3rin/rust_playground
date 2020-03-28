use std::{env, fs, io};
use std::fs::File;
use walkdir::{DirEntry, WalkDir};
use std::io::{Write, BufReader, BufRead, Error};


fn is_md(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with("md"))
        .unwrap_or(false)
}

fn walk_dir(filepath: &str, replace_dir: &str, heand_title: &str) -> io::Result<()> {
    let mut out = String::new();
    out += &("# ".to_string() + heand_title + "\n");

    let current_dir = env::current_dir()?;
    let current_str = current_dir.as_path().to_str().unwrap();
    let walker = WalkDir::new(current_str).into_iter();
    for entry in walker {
        let dir = entry?;
        if fs::metadata(dir.path())?.is_file() && is_md(&dir) {
            out += &("\n## title\n".to_string() + &dir.path().to_str().unwrap().replace(current_str, replace_dir) + "\n")
        }
    }
    write_contents(filepath, &out)?;
    Ok(())
}

fn write_contents(filepath: &str, out: &str) -> io::Result<()> {
    let mut f = File::create(filepath)?;
    f.write_all(out.as_bytes())?;
    Ok(())
}

fn main() {
    match walk_dir("output.md", "", "Blog post list") {
        Ok(()) => println!("success"),
        Err(msg) => println!("failure: {}", msg),
    }
}
