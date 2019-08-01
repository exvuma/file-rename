use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)] //attribute
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The string to replace filename with 
    replace: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// read through a recursively directory read in filenames and rename if match pattern
fn visit_dirs(dir: &Path, pattern: &String,  replace: &String) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let mut final_path = entry.path();
            let ref_path = final_path.clone();
            if ref_path.is_dir() {
                visit_dirs(&ref_path, pattern, &replace)?;
            } else {
                if ref_path.to_string_lossy().contains(pattern) {
                    println!("{:?}", &final_path);
                    final_path.set_file_name(replace);
                    println!("{:?}", &final_path);
                } else {
                    println!("{:?}", "didn't match");
                    println!("{:?}", &ref_path);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let args = Cli::from_args();
    visit_dirs(&args.path, &args.pattern, &args.replace);
}
