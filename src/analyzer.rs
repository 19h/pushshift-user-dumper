use std::fs::{DirEntry, File};
use std::io::{BufReader, Read};
use std::path::Path;

use zstd::Decoder;

use serializer::deserialize;
use text::text_item::TextItem;

mod text;
mod serializer;

fn run_for_file(path: &Path) {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();

    println!("name: {}", name);

    let mut file = File::open(path).unwrap();

    let mut decoder =
        Decoder::new(&mut file).unwrap();

    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf).unwrap();
    //file.read_to_end(&mut buf).unwrap();

    let ti =
        TextItem {
            word_freqs: deserialize(&buf),
        };

    let wc = ti.word_freqs.len();
    let (f_min, f_max) =
        ti.word_freqs
            .iter()
            .fold((std::f64::MAX, 0f64), |(min, max), (_, f)| {
                (min.min(*f as f64) as f64, max.max(*f as f64) as f64)
            });

    dbg!(wc, f_min, f_max);

    let mut wf =
        ti.word_freqs
            .iter()
            .collect::<Vec<_>>();

    wf.sort_by(|a, b| b.1.cmp(a.1));

    wf.iter()
        .take(128 * 128)
        .for_each(|(k, v)| {
            println!(
                "{:?}: {}",
                String::from_utf8(k.to_vec()),
                ((**v as f64 - f_min) / (f_max - f_min)),
            );
        });
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
