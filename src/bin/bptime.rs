use std::fs::File;
use std::io::{self, stdin, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    /// input file
    #[structopt(parse(from_os_str), short = "f")]
    file: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::from_args();
    let mut input = String::new();
    if let Some(file) = args.file {
        let mut file_handle = File::open(file)?;
        file_handle.read_to_string(&mut input)?;
    } else {
        stdin().lock().read_to_string(&mut input)?;
    };

    let lines = input.lines();

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
