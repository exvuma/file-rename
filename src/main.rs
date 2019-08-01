use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use structopt::StructOpt;
/// Search for a pattern in a file and display the lines that contain it.
// !#allow(unused)] //clippy

#[derive(StructOpt)] //attribute
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// one possible implementation of walking a directory only visiting files
// fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
fn visit_dirs(dir: &Path, pattern: &String) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let mut final_path = entry.path();
            let ref_path = final_path.clone();
            // println!("{:?}", "trying to match");
            // println!("{:?}", &pattern);
            if ref_path.is_dir() {
                visit_dirs(&ref_path, pattern)?;
            } else {
                // final_path.set_file_name("debug2");
                // let tempPath =  std::path::PathBuf::new(path);
                if ref_path.to_string_lossy().contains(pattern) {
                    println!("{:?}", &final_path);
                    // &tempfinal_path.set_file_name("blah.txt");
                    // let filename = final_path.file_nam e;
                    final_path.set_file_name("debug2");
                    println!("{:?}", &final_path);
                // fs::rename(&path, "debug2")?;
                } else {
                    println!("{:?}", "didn't match");
                    println!("{:?}", &ref_path);
                }
            }
        }
    }
    Ok(())
}

fn blah(path: String) {}
fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;
    let args = Cli::from_args();
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    //Read in content of file
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    let g = visit_dirs(&args.path, &args.pattern);
    let mut writer = BufWriter::new(stdout.lock());
    // say(out, width, &mut writer).unwrap();
    // say(&args.pattern.as_bytes(), width, &mut writer);
}
