use clap::{App, Arg};
use pretty_bytes::converter::convert;
use std::{fs, path::PathBuf};

fn main() {
    let matches = App::new("avg-file-size")
        .version("0.1")
        .arg(
            Arg::with_name("path")
                .value_name("DIRECTORIES")
                .help("one or more directories to calculate avg file size in")
                .takes_value(true)
                .required(false)
                .multiple(true),
        )
        .arg(
            Arg::with_name("human-readable")
                .short("h")
                .long("human-readable")
                .value_name("human-readable")
                .help("output in human-readable format e.g. 10 kB, 5.1 gB")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("round")
                .short("r")
                .long("round")
                .value_name("round")
                .help("round to the nearest kilobyte")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let mut dirs: Vec<&str> = matches.values_of("path").unwrap_or_default().collect();
    if dirs.is_empty() {
        dirs.push("./");
    }

    let human_readable: bool = matches.is_present("human-readable");
    let round: bool = matches.is_present("round");

    dirs.iter().for_each(|dir| {
        let pathbuf: PathBuf = PathBuf::from(dir);
        let (total_size, file_count) = avg_file_size(pathbuf);

        let mut avg_size: f64 = total_size / file_count;

        if round {
            avg_size = avg_size.round();
        }

        if human_readable {
            println!(
                "{}  {}",
                convert((avg_size) * 1000f64).replace(" ", ""),
                dir
            );
        } else {
            println!("{}  {}", avg_size, dir);
        }
    });
}

fn avg_file_size(dir: PathBuf) -> (f64, f64) {
    let mut file_count: f64 = 0.0;
    let mut total_size: f64 = 0.0;

    if let Ok(files) = fs::read_dir(dir) {
        files.for_each(|file| if let Ok(file) = file {
            if let Ok(metadata) = file.metadata() {
                if metadata.is_file() {
                    //println!("{:?} : {}", file.path(), metadata.len());
                    file_count += 1.0;
                    total_size += (metadata.len() / 1000) as f64;
                } else if metadata.is_dir() {
                    let (sub_total_size, sub_file_count) = avg_file_size(file.path());
                    file_count += sub_file_count;
                    total_size += sub_total_size;
                }
            }
        });
    }
    //println!("{} {}", total_size, file_count);
    
    (total_size as f64, file_count as f64)
}
