#[macro_use]
extern crate serde;
extern crate clap;
extern crate quick_xml;

mod csv_writer;
mod jira_export_struct;
mod out_writer;
mod xml_reader;

use anyhow::{bail, Result};
use clap::{app_from_crate, Arg, ArgMatches};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::path::Path;
use jira_export_struct::Rss;

enum OutFormat {
    CSV,
    STDOUT,
}

fn get_args() -> ArgMatches {
    app_from_crate!()
        .arg(
            Arg::new("FILE")
                .about("Input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("csv")
                .long("csv")
                .about("Output to csv file next to the input file"),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .about("Enable debug output"),
        )
        .get_matches()
}

fn init_logger(debug: bool) {
    let level = match debug {
        true => LevelFilter::Trace,
        false => LevelFilter::Info,
    };
    SimpleLogger::new().with_level(level).init().unwrap()
}

fn process_file(input_file: &Path, out_format: OutFormat) -> Result<()> {
    let rss: Rss = xml_reader::read_file(input_file)?;
    log::debug!("Parsed input file {:#?}:\n{:#?}", input_file, rss);

    match out_format {
        OutFormat::CSV => csv_writer::write_file(rss, input_file),
        OutFormat::STDOUT => out_writer::write_out(rss),
    }
}

fn main() -> Result<()> {
    let matches = get_args();

    init_logger(matches.is_present("debug"));

    match matches.value_of("FILE") {
        Some(f) => {
            let out_format = if matches.is_present("csv") {
                OutFormat::CSV
            } else {
                OutFormat::STDOUT
            };
            process_file(Path::new(f), out_format)
        }
        None => bail!("Missing file argument!"),
    }
}
