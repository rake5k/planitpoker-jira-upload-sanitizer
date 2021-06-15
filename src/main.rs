#[macro_use]
extern crate serde;
extern crate clap;
extern crate quick_xml;

use anyhow::{bail, Result};
use clap::{app_from_crate, Arg};
use csv::WriterBuilder;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "rss")]
struct Rss {
    channel: Channel,
}

#[derive(Deserialize, Serialize, Debug)]
struct Channel {
    #[serde(rename = "item")]
    items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Item {
    title: String,
    link: String,
}

fn init_logger(debug: bool) {
    let level = match debug {
        true => LevelFilter::Trace,
        false => LevelFilter::Info,
    };
    SimpleLogger::new().with_level(level).init().unwrap()
}

fn process_file(path: &Path, as_csv: bool) -> Result<()> {
    let rss: Rss = read_file(path)?;
    log::debug!("Parsed input file {:#?}:\n{:#?}", path, rss);

    if as_csv {
        write_file(rss, &get_out_file(path))
    } else {
        write_out(rss)
    }
}

fn read_file(path: &Path) -> Result<Rss> {
    match File::open(path) {
        Ok(f) => Ok(quick_xml::de::from_reader(BufReader::new(f))?),
        Err(_) => bail!("Cannot read file in path '{}'", path.display()),
    }
}

fn get_out_file(path: &Path) -> PathBuf {
    let out_file_name = path.with_extension("safe.csv");
    path.with_file_name(out_file_name)
}

fn write_file(rss: Rss, path: &Path) -> Result<()> {
    match File::create(path) {
        Ok(f) => {
            eprint!("Writing {:?}...", path);
            let mut wtr = WriterBuilder::new()
                .has_headers(false)
                .from_writer(BufWriter::new(f));
            for item in rss.channel.items {
                wtr.serialize(make_anchor(&item.title, &item.link))?;
            }
            eprintln!(" done!");
            Ok(())
        }
        Err(_) => bail!("Cannot write file in path '{}'", path.display()),
    }
}

fn make_anchor(text: &str, link: &str) -> String {
    format!("<a href='{}'>{}</a>", link, text)
}

fn write_out(rss: Rss) -> Result<()> {
    eprintln!("[ID] TITLE, LINK");
    eprintln!("----------------");
    for item in rss.channel.items {
        println!("{}, {}", remove_commas(&item.title), cut_comma(&item.link));
    }
    Ok(())
}

fn remove_commas(text: &str) -> String {
    text.replace(',', "")
}

fn cut_comma(text: &str) -> &str {
    text.split(',').nth(0).unwrap_or("-")
}

fn main() -> Result<()> {
    let matches = app_from_crate!()
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
        .get_matches();

    init_logger(matches.is_present("debug"));

    match matches.value_of("FILE") {
        Some(f) => process_file(Path::new(f), matches.is_present("csv")),
        None => bail!("Missing file argument!"),
    }
}
