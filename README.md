# `afs`

A command-line utility to calculate the average size of all files in a directory, recursively. Results are in Kilobytes.

## Usage

```
USAGE:
    afs [FLAGS] [path]

FLAGS:
        --help              Prints help information
    -h, --human-readable    output in human-readable format e.g. 10 kB, 5.1 gB
    -V, --version           Prints version information

ARGS:
    <path>    directory to calculate avg file size in
```

## Building

Build with `cargo build`. Requires Rust to be installed, along with cargo.