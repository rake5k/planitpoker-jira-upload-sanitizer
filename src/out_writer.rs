use anyhow::Result;
use crate::Rss;

pub fn write_out(rss: Rss) -> Result<()> {
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
