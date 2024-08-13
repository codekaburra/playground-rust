
use std::env::args;
use std::time::SystemTime;
use std::fs;
use std::collections::HashMap;

use pdf::file::{FileOptions};
use pdf::object::*;
use pdf::primitive::Primitive;
use pdf::error::PdfError;
use pdf::enc::StreamFilter;
use std::fs::File;
use std::io::prelude::*;
use reqwest;

#[tokio::main]
fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn main1() -> Result<(), PdfError> {
    let path = args().nth(1).expect("no file given");
    println!("read: {}", path);
    let now = SystemTime::now();

    let file = FileOptions::cached().open(&path).unwrap();
    if let Some(ref info) = file.trailer.info_dict {
    println!("info: {}", info);
    let title = info.get("Title").and_then(|p| p.to_string_lossy().ok());
        let author = info.get("Author").and_then(|p| p.to_string_lossy().ok());

        let descr = match (title, author) {
            (Some(title), None) => title,
            (None, Some(author)) => format!("[no title]  {}", author),
            (Some(title), Some(author)) => format!("{}  {}", title, author),
            _ => "PDF".into()
        };
        println!("{}", descr);
    }

    if let Ok(elapsed) = now.elapsed() {
        println!("Time: {}s", elapsed.as_secs() as f64
                 + elapsed.subsec_nanos() as f64 * 1e-9);
    }
    Ok(())
}