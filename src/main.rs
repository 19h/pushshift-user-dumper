#![feature(slice_internals)]

extern crate core;

use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::ops::AddAssign;
use std::path::Path;

use kdam::{BarExt, Column, RichProgress, tqdm};
use kdam::term::Colorizer;
use rayon::prelude::*;
use ruzstd::{FrameDecoder, StreamingDecoder};
use serde::{Deserialize, Serialize};

use crate::serializer::serialize;
use crate::text::text_item::{PooMap, TextItem};

pub mod text;
pub mod serializer;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub author: String,
    pub body: String,
    //#[serde(rename = "created_utc")]
    //pub created_utc: u64,
}

fn read_until<R: BufRead + ?Sized>(r: &mut R, delim: u8, buf: &mut Vec<u8>) -> Result<usize, Error> {
    unsafe {
        let mut read = 0;
        loop {
            let (done, used) = {
                let available = match r.fill_buf() {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };
                match core::slice::memchr::memchr(delim, available) {
                    Some(i) => {
                        buf.extend_from_slice(&available[..=i]);
                        (true, i + 1)
                    }
                    None => {
                        buf.extend_from_slice(available);
                        (false, available.len())
                    }
                }
            };
            r.consume(used);
            read += used;
            if done || used == 0 {
                return Ok(read);
            }
        }
    }
}

fn run_for_file(path: &Path, username: &str) {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();

    let mut dec = FrameDecoder::new();

    dec.init(File::open(path).unwrap()).unwrap();

    let mut ti = TextItem::new();

    let size = dec.content_size().unwrap_or(0) as usize;

    println!("size: {} GB", size as f64 / 1024.0 / 1024.0 / 1024.0);

    let mut pb = RichProgress::new(
        tqdm!(
            total = size,
            unit_scale = true,
            unit_divisor = 1024,
            unit = "B"
        ),
        vec![
            Column::Spinner(
                "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
                    .chars()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                80.0,
                1.0,
            ),
            Column::text("[bold blue]?"),
            Column::Bar,
            Column::Percentage(1),
            Column::text("•"),
            Column::CountTotal,
            Column::text("•"),
            Column::Rate,
            Column::text("•"),
            Column::RemainingTime,
        ],
    );

    let hint_needle =
        &*format!(
            ":\"{}\"",
            username,
        )
        .as_bytes()
        .to_vec();

    pb.write(format!("Loading zstd for file {}...", name).colorize("bold blue"));

    let mut file = File::open(path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut file = BufReader::new(buf.as_slice());

    let mut decoder =
        BufReader::new(StreamingDecoder::new(&mut file).unwrap());

    pb.write(format!("Processing {}...", name).colorize("green"));

    let mut len_read = 0usize;
    let mut i = 0u64;

    let per_iter = 10000usize;

    let mut err_cnt = 0usize;

    'a: loop {
        //let mut comments = Vec::<String>::new();
        let mut comments = vec![b'['];
        let mut bucket = Vec::new();

        'b: for k in 0..per_iter {
            let mut line = Vec::new();

            if let Err(x) = read_until(&mut decoder, b'\n', &mut line) {
                dbg!(x);

                break 'a;
            }

            if line.len() == 0 {
                err_cnt += 1;

                if err_cnt > 10 {
                    break 'a;
                }

                break 'b;
            }

            len_read += line.len();
            i += 1;

            if i < 1e9 as u64 {
                i += 1;


                continue;
            }

            bucket.push(line);
        }

        let mut buckets =
            bucket
                .par_iter()
                .filter_map(|x| {
                    let mut is_match = false;

                    let mut p = 0;
                    let q =  x.len() - hint_needle.len();

                    for i in 0..q {
                        if x[i] == hint_needle[p] {
                            p += 1;

                            if p == hint_needle.len() {
                                is_match = true;
                                break;
                            }
                        } else {
                            p = 0;
                        }
                    }

                    if is_match {
                        // append comma to x slice before returning
                        Some([&x, [b','].as_slice()].concat())
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<_>>();

        comments.append(
            &mut buckets,
        );

        if i % 100000 == 0 {
            pb.update_to(len_read);
        }

        if comments.len() == 1 {
            continue;
        }

        let comment_len = comments.len();

        dbg!(String::from_utf8(comments.clone()).unwrap());
        if &comments[comment_len - 5..] == &[b',', b']'] {
            comments[comment_len - 1] = b' ';
        }

        comments.push(b']');

        let xcomments =
            simd_json::from_slice::<Vec<Comment>>(comments.as_mut_slice())
                .map(|x|
                    x
                        .into_iter()
                        .map(|x| (x.author, x.body))
                        .collect::<Vec<_>>()
                );

        if xcomments.is_err() {
            dbg!(String::from_utf8(comments[comments.len() - 50..].to_vec()).unwrap());

            err_cnt += 1;

            if err_cnt > 10 {
                break 'a;
            }

            continue;
        }

        let xcomments = xcomments.unwrap();

        ti.ingest(
            &xcomments
                .par_iter()
                .filter(|(author, _)| author == username)
                .map(|(_, comment)| TextItem::process_alt(comment))
                .fold(
                    || PooMap::new(),
                    |mut acc, freqs| {
                        dbg!(&freqs);

                        for (word, freq) in freqs.iter() {
                            acc.entry(word.clone())
                                .or_insert(0)
                                .add_assign(*freq);
                        }

                        acc
                    },
                )
                .reduce(
                    || PooMap::new(),
                    |mut acc, mut freqs| {
                        for (word, freq) in freqs.iter() {
                            acc.entry(word.clone())
                                .or_insert(0)
                                .add_assign(*freq);
                        }

                        acc
                    },
                ),
        );
    }

    let mut file =
        File::create(
            path
                .clone()
                .with_file_name(
                    format!("{}.{}.freqs", &name, username),
                )
        ).unwrap();

    let val = serialize(&ti.word_freqs);
    let val = zstd::encode_all(val.as_slice(), 20).unwrap();

    file.write_all(&val).unwrap();
}

fn main() {
    // find folder located at first argument
    let path = std::env::args().nth(1).expect("No path provided");
    let path = Path::new(&path);

    let username = std::env::args().nth(2).expect("No username provided");

    // find all files in folder
    let files = std::fs::read_dir(path).expect("Could not read directory");

    // filter for files ending with .zst
    let mut files =
        files
            .filter_map(|f| f.ok())
            .filter(|f| {
                f.path()
                    .extension()
                    .map(|ext| ext == "zst")
                    .unwrap_or(false)
            })
            .collect::<Vec<DirEntry>>();

    files.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));

    files
        .iter()
        .for_each(|f| {
            run_for_file(&f.path(), &username);
        });
}
