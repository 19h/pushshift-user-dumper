use std::fs::{DirEntry, File};
use std::io::{BufReader, Read};
use std::path::Path;

use ruzstd::{FrameDecoder, StreamingDecoder};

use crate::text::text_item::TextItem;

mod text;

fn run_for_file(path: &Path) {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();

    let mut dec = FrameDecoder::new();

    dec.init(File::open(path).unwrap()).unwrap();

    let size = dec.content_size().unwrap_or(0) as usize;

    println!("size: {} GB", size as f64 / 1024.0 / 1024.0 / 1024.0);

    println!("name: {}", name);

    let mut file = File::open(path).unwrap();
    let mut decoder =
        StreamingDecoder::new(&mut file).unwrap();

    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf).unwrap();

    let ti: TextItem = bincode::deserialize(&buf).unwrap();

    println!("ti: {:?}", ti);
}

fn main() {
    // find folder located at first argument
    let path = std::env::args().nth(1).expect("No path provided");
    let path = std::path::Path::new(&path);

    // find all files in folder
    let files = std::fs::read_dir(path).expect("Could not read directory");

    // filter for files ending with .zst
    let mut files =
        files
            .filter_map(|f| f.ok())
            .filter(|f| {
                f.path()
                    .extension()
                    .map(|ext| ext == "freqs")
                    .unwrap_or(false)
            })
            .collect::<Vec<DirEntry>>();

    files.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));

    files
        .iter()
        .for_each(|f| {
            run_for_file(&f.path());
        });
}
