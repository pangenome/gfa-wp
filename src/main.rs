use clap::{App, Arg};
use std::fs;
use std::io::{self, BufReader, BufRead};

fn main() -> Result<(), String> {

    let matches = App::new("gfa-wp")
        .version("0.1.0")
        .author("Glenn Hickey <glenn.hickey@gmail.com>")
        .about(concat!(
            "\nConvert GFA W-lines to P-lines\n"
        ))
        .arg(
            Arg::with_name("gfa_path")
                .value_name("GFA_PATH")
                .help("(uncompressed) GFA or - for stdin")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("separator")
                .value_name("SEPARATOR")
                .help("token used to ")
                .default_value("#")
                .takes_value(true),
        )
        .get_matches();

    let gfa_path = matches.value_of("gfa_path").unwrap();
    let sep = matches.value_of("separator").unwrap();

    let reader: Box<dyn BufRead>;
    if gfa_path == "-" {
        reader = Box::new(BufReader::new(io::stdin()));
    } else {
        reader = Box::new(BufReader::new(fs::File::open(gfa_path).unwrap()));
    }

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                if line.starts_with("W\t") {
                    let toks: Vec<&str> = line.split('\t').collect();
                    if toks.len() != 7 {
                        panic!("W line with {} tokens found, expected 7: {}", toks.len(), line);
                    }
                    let p_name = toks[1..4].join(sep) + ":" + toks[4] + "-" + toks[5];
                    print!("P\t{}\t", p_name);
                    let walk = toks[6].as_bytes();
                    let mut i = 1;
                    let mut last = 1;
                    let mut orient = &walk[0];
                    while i < walk.len() {
                        while i < walk.len() && (walk[i] as char).is_digit(10) {
                            i += 1;
                        }
                        let strand = if *orient == '>' as u8 { '+' } else { '-' };
                        let node = String::from_utf8(walk[last..i].to_vec()).unwrap();
                        if last > 1 {
                            print!(",");
                        }
                        print!("{}{}", node, strand);
                        if i < walk.len() { orient = &walk[i]; }
                        i += 1;
                        last = i;
                    }
                    print!("\t*\n");
                } else {
                    println!("{}", line);
                }
            },
            Err(why) => {
                panic!("Could not parse line: {}", why);
            },
        }
    }

    Ok(())
}
