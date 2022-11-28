use std::collections::{BTreeMap, HashMap};
use std::iter::Cloned;
use std::ops::AddAssign;
use std::str::SplitWhitespace;
use lazy_static::lazy_static;

use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use super::EN_TOKENIZER;

pub type PooMap = BTreeMap<Vec<u8>, u64>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextItem {
    pub word_freqs: PooMap,
}

impl TextItem {
    pub fn new() -> Self {
        Self {
            word_freqs: PooMap::new(),
        }
    }

    pub fn ingest(&mut self, other: &PooMap) {
        for (word, freq) in other.iter() {
            if word.len() > 25 {
                //continue;
            }

            self.word_freqs
                .entry(word.clone())
                .or_insert(0)
                .add_assign(*freq);
        }
    }

    #[inline(always)]
    pub fn process(text: &str) -> PooMap {
        EN_TOKENIZER
            .pipe(
                &text
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                    .collect::<String>(),
            )
            .collect::<Vec<_>>()
            .iter()
            .map(|x|
                x
                    .tokens()
                    .iter()
                    .map(|i|
                        i
                            .word()
                            .text()
                            .as_str()
                            .to_string()
                    ).collect::<Vec<_>>()
            )
            .flatten()
            .fold(
                PooMap::new(),
                |mut acc, word| {
                    *acc.entry(word.trim().as_bytes().to_vec()).or_insert(0) += 1u64;

                    acc
                },
            )
    }

    #[inline(always)]
    pub fn process_alt(text: &str) -> PooMap {
        text
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
            .split_whitespace()
            .fold(
                PooMap::new(),
                |mut acc, word| {
                    *acc.entry(word.trim().as_bytes().to_vec()).or_insert(0) += 1u64;

                    acc
                },
            )
    }
}

unsafe impl Send for TextItem {}

unsafe impl Sync for TextItem {}