use anyhow::{bail, Result};
use csv::WriterBuilder;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use crate::Rss;

pub fn write_file(rss: Rss, path: &Path) -> Result<()> {
    let out_file = get_out_file(path);
    match File::create(&out_file) {
        Ok(f) => {
            eprint!("Writing {:?}...", out_file);
            let mut wtr = WriterBuilder::new()
                .has_headers(false)
                .from_writer(BufWriter::new(f));
            for item in rss.channel.items {
                wtr.serialize(make_anchor(&item.title, &item.link))?;
            }
            eprintln!(" done!");
            Ok(())
        }
        Err(_) => bail!("Cannot write file in path '{}'", out_file.display()),
    }
}

fn make_anchor(text: &str, link: &str) -> String {
    format!("<a href='{}'>{}</a>", link, text)
}

fn get_out_file(path: &Path) -> PathBuf {
    let out_file_name = path.with_extension("out.csv");
    path.with_file_name(out_file_name)
}
