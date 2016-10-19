#[macro_use]
extern crate clap;
extern crate regex;

use std::str::FromStr;
use std::fs;
use std::io::{BufReader, BufRead, Read};
use regex::Regex;
use clap::{Arg, App};


pub fn validate_path(path: String) -> Result<(), String> {
    if fs::metadata(&path).is_ok() {
        if path.to_lowercase().ends_with(".rpp") {
            Ok(())
        } else {
            Err(String::from("Input file must have .rpp extension!"))
        }
    } else {
        Err(String::from("Input file doesn't exist"))
    }
}

pub fn validate_i32(value: String) -> Result<(), String> {
    value.parse::<i32>()
        .map(|_| ())
        .map_err(|_| "Supplied parameter is not a valid number!".to_owned())
}

fn main() {

    let matches = App::new("rpp-nudger")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Process Reaper DAW project files. Nudge all items audio content leaving all \
                envelopes in place. Program parses .rpp file and changes SOFFS item parameter. \
                Ignores reverse state!")
        .arg(Arg::with_name("INPUT")
            .required(true)
            .help("Path to the input file to process.")
            .index(1)
            .validator(validate_path))
        .arg(Arg::with_name("samples")
            .required(true)
            .help("Nudge amount in samples. Can be negative")
            .index(2)
            .validator(validate_i32))
        .get_matches();

    let file = fs::File::open(matches.value_of("INPUT").unwrap().to_string()).unwrap();
    let nudge = value_t!(matches, "samples", i32).unwrap();
    let mut reader = BufReader::new(file);
    let mut sample_rate = 44100;

    let re = Regex::new(r"( *)SOFFS *([0-9.]*)").unwrap();
    let re_srate = Regex::new(r" *SAMPLERATE *(\d+)").unwrap();

    for line in reader.by_ref().lines() {
        let l = line.unwrap();
        println!("{}", l);
        if let Some(cap) = re_srate.captures(&l) {
            sample_rate = usize::from_str(cap.at(1).unwrap()).unwrap();
            break;
        }
    }

    for line in reader.lines() {
        let l = line.unwrap();

        match re.captures(&l) {
            Some(cap) => {
                let soffs_old = f64::from_str(cap.at(2).unwrap()).unwrap();
                let soffs_new_str =
                    format!("{:.*}", 14, soffs_old + (nudge as f64 / sample_rate as f64));
                println!("{}SOFFS {}", cap.at(1).unwrap_or(""), soffs_new_str);
            }
            None => {
                println!("{}", l);
            }
        }

    }

}
