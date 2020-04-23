// yes
// written to work on Unix machines

extern crate getopts;

use getopts::Options;
use std::env;


fn usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [STRING]", program);
    print!("{}", opts.usage(&brief));

}


fn yes(s: &str){
    loop {
        println!("{}", s);
    }
}


fn main() {

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
        return;
    }

    let mut strs: Vec<String> = Vec::new();
    if !matches.free.is_empty() {
        strs = matches.free.clone();
    } else {
       strs.push(String::from("y")) ;
    }

    // join requires collection type to be &str
    let s: Vec<&str> = strs.iter()
        .map( |s| s.as_str()).collect();

    yes(&s.join(" "));
}
