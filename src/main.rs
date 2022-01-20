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
                        panic!("W line with {} tokesn found, expected 7: {}", toks.len(), line);
                    }
                    let p_name = toks[1..6].join(sep);
                    print!("P\t{}\t", p_name);
                    let steps: Vec<&str> = toks[6].split_inclusive(|c| c == '>' || c == '<').collect();
                    for i in 1..steps.len() {
                        let strand = if steps[i-1].chars().nth(0).unwrap() == '>' {'+'} else {'-'};
                        let node = &steps[i][0..steps[i].len()-1];
                        if i > 1 {
                            print!(",");
                        }
                        print!("{}{}", node, strand);
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
