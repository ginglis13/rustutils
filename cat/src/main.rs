// cat
// written to work on Unix machines

extern crate getopts;

use getopts::Options;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;



fn usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [OPTION] FILE", program);
    print!("{}", opts.usage(&brief));

}

fn cat(files: &Vec<String>) -> io::Result<()>{
    for f in files {
        let file = File::open(f)?;
        let f1 = BufReader::new(file);
        for line in f1.lines() {
            println!("{}", line.unwrap());
        }

    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {

    // Read argv
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    // Create Options
    let mut opts = Options::new();
    opts.optflag("h", "help", "print help information");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { usage(&program_name, &opts); panic!(e.to_string()) }
    };

    if matches.opt_present("h") {
        usage(&program_name, &opts);
        return Ok(());
    }

    let files: Vec<String>;
    if !matches.free.is_empty() {
        files = matches.free.clone();
    } else {
        files = [String::from("/dev/stdin")].to_vec();
    }

    return cat(&files);
}
