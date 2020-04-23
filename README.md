# rustutils

A few basic coreutils implementations written in Rust. Contributions welcome.

## Building/Running

`cargo build` to build all binaries, then `./target/debug/$CMD` to run.

If you want to only build one binary: `cargo build --bin <NAME>`.

## Utils

All utils with at least a base implementation are listed below. Each util
has a section with TODO's that are either missing options or implementationdetails for fixing existing options to match coreutils behavior.

### ls
```
Usage: ./target/debug/ls [OPTIONS] FILE

Options:
    -h, --help          print help information
    -A                  list all files and directories, including dotfiles,
                        but not '.' and '..'
    -l                  use a long listing format
    -R                  recursively list contents of directory

```

**Options left/todo**
- recursive doesn't produce same style output as coreutils
- color file types
- custom sort
- -a: show hidden including '.' and '..'
- and many many more...

### cat

Pretty much just the base implementation is in place.

- 'h', '--help'

**Options left/todo**
- read from stdin if no file provided
- show ends
- show tabs
- show all

### head

```
Usage: ./target/debug/head [OPTION] FILE

Options:
    -h, --help          print help information
    -c, --bytes NUM     print the first NUM bytes of each file
    -n, --lines NUM     print the first NUM lines instead of the first 10
    -q, --quiet         never print headers giving file names
    -v, --verbose       always print headers giving file names
```

**Options left/todo**
- read from stdin if no file provided

### tail

### pwd

### yes

:check:

### touch

### ...
