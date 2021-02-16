use std::{fs, path::PathBuf};
use clap::{App, Arg};

fn main() {
    let matches = App::new("avg-file-size")
        .version("0.1")
        .arg(Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("path")
                .help("directory to calculate avg file size in")
                .takes_value(true)
                .required(false)
                .index(1)).get_matches();
    
    let path: &str = matches.value_of("path").unwrap_or("./");
    let pathbuf: PathBuf = PathBuf::from(path);
    let (total_size, file_count) = avg_file_size(pathbuf);
    println!("{}  {}", path, total_size/file_count);
}

fn avg_file_size(dir: PathBuf) -> (f64, f64) {
    let mut file_count: f64 = 0.0;
    let mut total_size: f64 = 0.0;

    if let Ok(files) = fs::read_dir(dir) {
        for file in files {
            if let Ok(file) = file {
                if let Ok(metadata) = file.metadata() {
                    if metadata.is_file() {
                        //println!("{:?} : {}", file.path(), metadata.len());
                        file_count += 1 as f64;
                        total_size += (metadata.len()/1000 as u64) as f64;
                    } else if metadata.is_dir() {
                        let (sub_file_count, sub_total_size) = avg_file_size(file.path());
                        file_count += sub_file_count;
                        total_size += sub_total_size;
                        
                    }
                }
            }
        }
    }
    //println!("{} {}", total_size, file_count);
    (total_size as f64, file_count as f64)
}
