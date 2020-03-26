use std::path::PathBuf;
use structopt::StructOpt;
use std::io::{self, stdin, Read};

#[derive(StructOpt, Debug)]
struct Args {
    /// input file
    #[structopt(parse(from_os_str), short="f")]
    file: Option<PathBuf>,

    /// input content
    #[structopt(short="c")]
    content: Option<String>
}

fn main() -> io::Result<()> {
    let args = Args::from_args();
    println!("ARGS: {:?}", args);

    let mut buffer = String::new();
    stdin().lock().read_to_string(&mut buffer)?;
    println!("STDIN: {}", buffer);
    Ok(())
}
