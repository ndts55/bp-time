use bp_time::Summary;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, stdin, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    /// optional input file
    #[structopt(parse(from_os_str), short = "f")]
    file: Option<PathBuf>,

    /// all flag
    #[structopt(short, long)]
    all: bool,
}

fn main() -> io::Result<()> {
    let args = Args::from_args();
    if args.all {
        print_all()
    } else {
        print_one(args.file)
    }
}

fn print_one(file: Option<PathBuf>) -> io::Result<()> {
    let mut input = String::new();
    if let Some(file) = file {
        let mut file_handle = File::open(file)?;
        file_handle.read_to_string(&mut input)?;
    } else {
        stdin().lock().read_to_string(&mut input)?;
    };

    let s = Summary::new(input);

    println!("{}", s);

    Ok(())
}

fn print_all() -> io::Result<()> {
    let md_files = fs::read_dir(".")?
        .filter_map(|dir_entry| {
            dir_entry
                .ok()
                .map(|dir_entry| dir_entry.path())
                .filter(|path| path.is_file())
                .filter(|path| path.extension().filter(|&ext| ext == "md").is_some())
        })
        .collect::<Vec<PathBuf>>();

    let mut total = Summary::empty();
    for md_file in md_files {
        let name = format!("{}\n---", md_file.file_name().and_then(OsStr::to_str).unwrap_or(""));
        let mut input = String::new();
        let mut file = File::open(md_file)?;
        file.read_to_string(&mut input)?;
        let s = Summary::new(input);
        total.add(&s);
        println!("{}{}", name, s);
    }

    println!("==============");

    println!("Total{}", total);

    Ok(())
}
