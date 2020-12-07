#[macro_use]
extern crate serde;
extern crate quick_xml;

use csv::WriterBuilder;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

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
    link: String,
    status: String
}

fn readfile(path : &Path) -> Rss {
    let file = BufReader::new(File::open(path).unwrap());
    return quick_xml::de::from_reader(file).unwrap();
}

fn writefile(rss : Rss, source_file : &Path) -> Result<(), Box<dyn Error>> {
    let target_file_name = source_file.with_extension("safe.csv");
    let target_file = source_file.with_file_name(target_file_name);

    println!("Writing {:?}...", target_file);
    let file = BufWriter::new(File::create(target_file).unwrap());
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    for item in rss.channel.items {
        wtr.serialize(make_anchor(&item.title, &item.link))?;
    }
    Ok(())
}

fn make_anchor(text : &str, link : &str) -> String {
    format!("<a href='{}'>{}</a>", link, text)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\tcargo run --example print_pos -- input.xml");
        std::process::exit(1);
    }

    let source_file = Path::new(args[1].as_str());

    let rss : Rss = readfile(source_file);
    //println!("{:#?}", rss);

    writefile(rss, source_file).unwrap();
}

