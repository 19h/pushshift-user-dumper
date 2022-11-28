use std::ops::Sub;
use kdam::term::Colorizer;
use crate::text::text_item::PooMap;

#[inline(always)]
pub fn serialize(data: &PooMap) -> Vec<u8> {
    let mut serbuf = data.iter().collect::<Vec<_>>();

    let mut out = Vec::new();

    for (word, freq) in serbuf {
        out.extend_from_slice(word);

        match *freq {
            x if freq <= &255u64 => {
                out.extend_from_slice(&(x as u8).to_be_bytes());
                out.push(255);
                out.push(0);
            }
            x if freq <= &(u32::MAX as u64) => {
                out.extend_from_slice(&(x as u32).to_be_bytes());
                out.push(254);
                out.push(0);
            }
            x => {
                out.extend_from_slice(&(x as u64).to_be_bytes());
                out.push(253);
                out.push(0);
            }
        }
    }

    out
}

const DEBUG: bool = true;

pub fn deserialize(data: &[u8]) -> PooMap {
    let mut freq_vec = PooMap::new();

    let mut i = 0;
    let mut last_marker_pos = 0;

    while i < data.len() {
        if i == 0 {
            i += 1;

            continue;
        }

        if data[i] == 0 && i != 0 {
            let marker = data[i - 1];

            if marker != 255 && marker != 254 && marker != 253 {
                i += 1;

                continue;
            }

            let frame = &data[last_marker_pos + 1..i];

            let mut word_offset = 0;

            let freq =
                match marker {
                    255 => {
                        word_offset = 1;

                        frame[frame.len() - 2] as u64
                    }
                    254 => {
                        word_offset = 5;

                        let mut buf = [0u8; 4];
                        buf.copy_from_slice(&frame[frame.len() - 5..frame.len() - 1]);
                        u32::from_be_bytes(buf) as u64
                    }
                    253 => {
                        word_offset = 9;

                        dbg!(frame);

                        let mut buf = [0u8; 8];
                        buf.copy_from_slice(&frame[frame.len() - 9..frame.len() - 1]);
                        u64::from_be_bytes(buf)
                    }
                    _ => {
                        if DEBUG {
                            dbg!(last_marker_pos + 1..i, data.len(), format!("{:02X} / {}", data[i], data[i]), marker);
                            println!("{}", {
                                let x = &data[last_marker_pos - 5..(i + 5).min(data.len())];

                                x.iter()
                                    .enumerate()
                                    .map(|(i, y)| {
                                        if i == 6 || i == (x.len() - 6) {
                                            format!("{:02X}", *y).colorize("bold dark red")
                                        } else {
                                            format!("{:02X}", *y)
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            });
                        }

                        dbg!(frame, i, last_marker_pos);
                        println!(
                            "Invalid frame at [{} - {}] with len {}: should be 1, 4 or 8 bytes.",
                            last_marker_pos,
                            i,
                            frame.len(),
                        );

                        i += 1;

                        continue;
                    }
                };

            let word = frame[..frame.len() - word_offset].to_vec();

            freq_vec.insert(word, freq);

            last_marker_pos = i;
        }

        i += 1;
    }

    freq_vec
}