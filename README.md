# `afs`

A command-line utility to calculate the average size of all files in a directory, recursively. Results are in Kilobytes.

## Usage

```
USAGE:
    afs [FLAGS] [DIRECTORIES]...

FLAGS:
        --help              Prints help information
    -h, --human-readable    output in human-readable format e.g. 10 kB, 5.1 gB
    -r, --round             round to the nearest kilobyte
    -V, --version           Prints version information

ARGS:
    <DIRECTORIES>...    one or more directories to calculate avg file size in
```

## Building

Build with `cargo build`. Requires Rust to be installed, along with cargo.