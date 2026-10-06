use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use flate2::read::GzDecoder;
use md5::{Digest, Md5};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input.fastq.gz>", args[0]);
        process::exit(1);
    }

    let path = &args[1];

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening {}: {}", path, e);
            process::exit(1);
        }
    };

    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let mut hasher = Md5::new();

    let mut line_num: u64 = 0;

    for line in reader.lines() {
        line_num += 1;
        match line {
            Ok(content) => {
                if line_num % 4 == 1 {
                    // Match awk behavior: emit the line + trailing newline
                    hasher.update(content.as_bytes());
                    hasher.update(b"\n");
                }
            }
            Err(e) => {
                eprintln!("Error reading line {}: {}", line_num, e);
                process::exit(1);
            }
        }
    }

    let result = hasher.finalize();
    // Same format as md5sum
    println!("{:x}  -", result);
}
