// head
// written to work on Unix machines

extern crate getopts;

use getopts::Options;
use getopts::Matches;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::env;


fn usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [OPTION] FILE", program);
    print!("{}", opts.usage(&brief));

}


fn head(files: &Vec<String>, opts: &Matches) -> io::Result<()>{
    for f in files {
        let file = File::open(f)?;
        let reader = BufReader::new(file);

        if opts.opt_present("q") {
        } else if opts.opt_present("v") || files.len() > 1 {
            println!("==> {} <==", f);
        }

        if opts.opt_present("c") {
            // read c bytes

            // convert to int
            let _count = opts.opt_str("c").unwrap();
            let count = _count.parse::<usize>().unwrap();

            for b in reader.bytes().take(count) {
                // print bytes as chars
                print!("{}", b.unwrap() as char);
            }
        } else if opts.opt_present("n") {
            // read n lines

            // convert to int
            let _count = opts.opt_str("n").unwrap();
            let count = _count.parse::<usize>().unwrap();

            let mut lines = reader.lines();
            for _ in 0..count {
                // double unwrap Option of lines.next()
                // and Reulst of second .unwrap()
                println!("{}", lines.next().unwrap().unwrap());
            }

        } else {
            // read 10 lines
            let mut lines = reader.lines();

            for _ in 0..10 {
                // print unwrapped result of lines.next()
                // if iteration succeeds
                match lines.next() {
                    Some(x) => println!("{}", x.unwrap()),
                    None => print!(""),
                }
            }
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
    opts.optopt("c", "bytes", "print the first NUM bytes of each file", "NUM");
    opts.optopt("n", "lines", "print the first NUM lines instead of the first 10", "NUM");
    opts.optflag("q", "quiet", "never print headers giving file names");
    opts.optflag("v", "verbose", "always print headers giving file names");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { usage(&program_name, &opts); panic!(e.to_string()) }
    };

    if matches.opt_present("h") {
        usage(&program_name, &opts);
        return Ok(());
    }

    let mut files: Vec<String> = Vec::new();
    if !matches.free.is_empty() {
        files = matches.free.clone();
    }

    return head(&files, &matches);
}
