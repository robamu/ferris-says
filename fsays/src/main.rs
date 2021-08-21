#![recursion_limit = "1024"]

extern crate clap;
extern crate ferris_says;
#[macro_use]
extern crate error_chain;

use clap::{App, Arg};
use ferris_says::*;
use std::fs::File;
use std::io::{stderr, stdin, stdout, BufReader, BufWriter, Read, Write};
use std::process::exit;

error_chain! {}

// Constants used for err messages
const ARGS: &str = "Invalid argument passed to fsays caused an error";
const INPUT: &str = "Failed to read input to the program";
const STDOUT: &str = "Failed to write stdout";
const STDERR: &str = "Failed to write stderr";

fn main() {
    if let Err(ref e) = run() {
        let stderr = &mut stderr();

        writeln!(stderr, "error: {}", e).expect(STDERR);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(STDERR);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(STDERR);
        }

        exit(1);
    }
}

fn run() -> Result<()> {
    let args = App::new("Ferris Says")
        .version("0.1")
        .author("Michael Gattozzi <mgattozzi@gmail.com>")
        .about("Prints out input text with Ferris the Rustacean")
        .arg(
            Arg::with_name("FILES")
                .long("files")
                .short("f")
                .help("Sets the input files to use")
                .required(false)
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("WIDTH")
                .long("width")
                .short("w")
                .help("Sets the width of the text box")
                .takes_value(true)
                .default_value("40")
                .required(false),
        )
        .arg(
            Arg::with_name("TEXT")
                .required(false)
                .multiple(true)
                .hidden(true),
        )
        .arg(
            Arg::with_name("SPEECH")
                .long("speech")
                .short("s")
                .help("Set speech mode")
                .takes_value(true)
                .default_value("say")
                .possible_values(&["say", "think"])
        )
        .arg(
            Arg::with_name("EYES")
                .long("eyes")
                .short("e")
                .help("Set eyes")
                .takes_value(true)
                .default_value("regular")
                .possible_values(&[
                    "regular", "greedy", "happy", "tired", "dead", "youth",
                    "paranoid", "crying"
                ])
        )
        .arg(
            Arg::with_name("SPEAKER")
                .long("speaker")
                .short("t")
                .help("Set antother speaker")
                .takes_value(true)
                .default_value("ferris")
                .possible_values(&[
                    "ferris", "clippy"
                ])
        )
        .get_matches();

    let width = args.value_of("WIDTH").unwrap().parse().chain_err(|| ARGS)?;

    let stdin = stdin();
    let stdout = stdout();

    let mode = match args.value_of("SPEECH").unwrap() {
        "say" => SpeechModes::Say,
        "think" => SpeechModes::Think,
        _ => SpeechModes::Say
    };

    let eyes = match args.value_of("EYES").unwrap() {
        "regular" => Eyes::RegularEyes,
        "greedy" => Eyes::GreedyEyes,
        "happy" => Eyes::HappyEyes,
        "tired" => Eyes::TiredEyes,
        "dead" => Eyes::DeadEyes,
        "youth" => Eyes::YouthfulEyes,
        "paranoid" => Eyes::ParanoidEyes,
        "crying" => Eyes::CryingEyes,
        _ => Eyes::RegularEyes
    };

    let speaker = match args.value_of("SPEAKER").unwrap() {
        "ferris" => Speaker::Ferris,
        "clippy" => Speaker::Clippy,
        _ => Speaker::Ferris
    };

    set_speaker(&speaker).expect("Could not set speaker");

    let cfg = FerrisConfig { mode, eyes };

    let mut writer = BufWriter::new(stdout.lock());

    if let Some(files) = args.values_of("FILES") {
        // Read in files and say them with Ferris
        let reader = files
            .map(|i| {
                let reader = BufReader::new(File::open(i).chain_err(|| INPUT)?);
                Ok(reader
                    .bytes()
                    .fold(Ok(Vec::new()), |a: Result<Vec<u8>>, b| {
                        if let Ok(mut a) = a {
                            a.push(b.chain_err(|| INPUT)?);
                            Ok(a)
                        } else {
                            a
                        }
                    })?)
            })
            .collect::<Vec<Result<Vec<u8>>>>();
        for i in reader {
            perform(&i?, width, &mut writer, &cfg).chain_err(|| STDOUT)?
        }

        Ok(())
    } else if let Some(other_args) = args.values_of("TEXT") {
        let s = other_args.collect::<Vec<&str>>().join(" ");
        perform(s.as_bytes(), width, &mut writer, &cfg).chain_err(|| STDOUT)?;
        Ok(())
    } else {
        let reader = BufReader::new(stdin.lock()).bytes().fold(
            Ok(Vec::new()),
            |a: Result<Vec<u8>>, b| {
                if let Ok(mut a) = a {
                    a.push(b.chain_err(|| INPUT)?);
                    Ok(a)
                } else {
                    a
                }
            },
        )?;
        perform(&reader, width, &mut writer, &cfg).chain_err(|| STDOUT)?;

        Ok(())
    }
}
