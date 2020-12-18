#[macro_use]
extern crate serde;
extern crate quick_xml;
extern crate clap;

use clap::{Arg, app_from_crate};
use csv::WriterBuilder;
use simple_logger::SimpleLogger;
use log::LevelFilter;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename="rss")]
struct Rss {
    channel: Channel
}

#[derive(Deserialize, Serialize, Debug)]
struct Channel {
    #[serde(rename="item")]
    items: Vec<Item>
}

#[derive(Deserialize, Serialize, Debug)]
struct Item {
    title: String,
    link: String
}

fn init_logger(debug: bool) {
    let level = match debug {
        true => LevelFilter::Trace,
        false => LevelFilter::Info,
    };
    SimpleLogger::new().with_level(level).init().unwrap()
}

fn read_file(path : &Path) -> Rss {
    let file = BufReader::new(File::open(path).unwrap());
    return quick_xml::de::from_reader(file).unwrap();
}

fn get_out_file(file: &Path) -> PathBuf {
    let out_file_name = file.with_extension("safe.csv");
    file.with_file_name(out_file_name)
}

fn write_file(rss : Rss, file : &Path) -> Result<(), Box<dyn Error>> {
    eprint!("Writing {:?}...", file);
    let file = BufWriter::new(File::create(file).unwrap());
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    for item in rss.channel.items {
        wtr.serialize(make_anchor(&item.title, &item.link))?;
    }
    eprintln!(" done!");
    Ok(())
}

fn make_anchor(text : &str, link : &str) -> String {
    format!("<a href='{}'>{}</a>", link, text)
}

fn write_out(rss : Rss) -> Result<(), Box<dyn Error>> {
    eprintln!("[ID] TITLE, LINK");
    eprintln!("----------------");
    for item in rss.channel.items {
        println!("{}, {}", remove_commas(&item.title), cut_comma(&item.link));
    }
    Ok(())
}

fn remove_commas(text : &str) -> String {
    text.replace(',', "")
}

fn cut_comma(text: &str) -> &str {
    text.split(',').nth(0).unwrap_or("-")
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::new("FILE")
            .about("Input file to use")
            .required(true)
            .index(1))
        .arg(Arg::new("csv")
            .long("csv")
            .about("Output to csv file next to the input file"))
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .about("Enable debug output"))
       .get_matches();

    init_logger(matches.is_present("debug"));

    let file = Path::new(matches.value_of("FILE").unwrap());
    let rss : Rss = read_file(file);
    log::debug!("Parsed input file {:#?}:\n{:#?}", file, rss);

    if matches.is_present("csv") {
        write_file(rss, &get_out_file(file)).unwrap();
    } else {
        write_out(rss).unwrap();
    }
}
