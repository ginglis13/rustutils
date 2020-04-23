// ls
// written to work on Unix machines

extern crate getopts;
extern crate chrono;

use getopts::Options;
use getopts::Matches;
use std::path::PathBuf;
use std::fs;
use std::env;
use std::fmt::Write;

use libc::{S_IRUSR, S_IWUSR, S_IXUSR,
           S_IRGRP, S_IWGRP, S_IXGRP,
           S_IROTH, S_IWOTH, S_IXOTH};

// for displaying permissions w/ -l
use std::os::unix::fs::MetadataExt;
//use std::os::unix::fs::PermissionsExt;
use chrono::{DateTime, Local};
//use std::time;


fn usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [OPTIONS] FILE", program);
    print!("{}", opts.usage(&brief));

}

// long_list
//
// for -l option, return metadata as String
fn long_list(path: &PathBuf) -> String {

    let mut perms = String::with_capacity(9);

    if let Ok(metadata) = path.metadata() {
        let datetime: DateTime<Local> = metadata.modified().unwrap().into();
        let mode= metadata.mode();

        // Check dir vs file
        perms.push( if path.is_dir() {'d'} else {'-'} );

        // Check perms
        perms.push( if (mode & S_IRUSR) != 0 {'r'} else {'-'} );
        perms.push( if (mode & S_IWUSR) != 0 {'w'} else {'-'} );
        perms.push( if (mode & S_IXUSR) != 0 {'x'} else {'-'} );
        perms.push( if (mode & S_IRGRP) != 0 {'r'} else {'-'} );
        perms.push( if (mode & S_IWGRP) != 0 {'w'} else {'-'} );
        perms.push( if (mode & S_IXGRP) != 0 {'x'} else {'-'} );
        perms.push( if (mode & S_IROTH) != 0 {'r'} else {'-'} );
        perms.push( if (mode & S_IWOTH) != 0 {'w'} else {'-'} );
        perms.push( if (mode & S_IXOTH) != 0 {'x'} else {'-'} );

        // TODO: width formatting for size should be based on larget file output
        write!(&mut perms, " {:5} {}", metadata.len(), datetime.format("%b %e %R")).unwrap();
    }
    perms
}


// ls
//
// read a directory given Options opts
// pass in matches flags to check if flag was present
fn ls(path: &PathBuf, options: &Matches) {

    // create vector of PathBuf
    //
    // https://users.rust-lang.org/t/how-to-convert-from-iterator-to-vector/4278
    // https://doc.rust-lang.org/stable/rust-by-example/error/iter_result.html
    //
    // would be good to check result of read_dir to ensure it is not attempting
    // to read a file...
    //
    // read dir into vector of PathBufs
    let mut entries: Vec<_> = fs::read_dir(path)
        // unwrap the Result of read_dir
        .unwrap()
        .map( |res|
              // unwrap the Result of iterating using map
              res.unwrap().path()
              // collect paths
              ).collect();

    // by default, ls sorts. in future - add custom sorting
    entries.sort();

    for e in entries {
        let _path = e.strip_prefix("./");
        let path: PathBuf;

        if _path.is_ok() {
            path = _path.unwrap().to_path_buf();
        } else {
            path = e.clone(); // copy over path, cannot borrow moved val
        }

        // get file name for checking opts and printing
        let file_name = path.file_name();
        let name = file_name.unwrap().to_string_lossy();

        if !options.opt_present("A") && name.starts_with(".") {
            continue;
        }

        if options.opt_present("l") {
            let md = long_list(&path);
            println!("{} {}", md, name);
        } else {
            print!("{} ", name);
        }

        if options.opt_present("R") && path.is_dir() {
            ls(&path.to_path_buf(), options);
        }
    }
}


fn main() {

    // Read argv
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    // Create Options
    let mut opts = Options::new();
    opts.optflag("h", "help", "print help information");
    opts.optflag("A", "", "list all files and directories, including dotfiles, but not '.' and '..'");
    opts.optflag("l", "", "use a long listing format");
    opts.optflag("R", "", "recursively list contents of directory");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { usage(&program_name, &opts); panic!(e.to_string()) }
    };

    if matches.opt_present("h") {
        usage(&program_name, &opts);
        return;
    }

    let mut target = ".".to_string();
    if !matches.free.is_empty() {
        target = matches.free[0].clone();
    }

    ls(&PathBuf::from(target), &matches);
    if !matches.opt_present("l") {
        println!();
    }
}
