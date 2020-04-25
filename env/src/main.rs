// env
// written to work on Unix machines

extern crate getopts;

use getopts::Options;

use std::env;



fn usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [OPTION] FILE", program);
    print!("{}", opts.usage(&brief));

}

fn env() {
    for (k,v) in env::vars() {
        println!("{}={}", k, v);
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

    env();
}
