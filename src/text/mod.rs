use std::collections::{HashMap, HashSet};
use std::io::BufReader;



use lazy_static::lazy_static;
use nlprule::tokenizer::Tokenizer;

pub mod text_item;

lazy_static! {
    pub(crate) static ref EN_TOKENIZER: Tokenizer = {
        let model = include_bytes!(concat!(env!("ASSET_DIR"), "/en_tokenizer.bin"));

        Tokenizer::from_reader(BufReader::new(&*model.to_vec())).unwrap()
    };

    pub static ref STOPWORDS: HashSet<String> =
        include_str!("./stopwords.txt")
            .lines()
            .map(|v| v.to_string())
            .collect();

    pub static ref SLANG_WORDS: HashMap<String, String> =
        include_str!("./slangwords.txt")
            .lines()
            .map(|v| {
                let r = v.split(",").collect::<Vec<&str>>();

                (r[0].to_string(), r[1].to_string())
            })
            .collect();

    pub static ref SPECIAL_CHARS: HashSet<char> =
        vec![
            '.',
            ',',
            '!',
            '?',
            ';',
            ':',
            '(',
            ')',
            '[',
            ']',
            '{',
            '}',
            '"',
            '\'',
            '/',
            '\\',
            '>',
            '<',
            '=',
            '+',
            '-',
            '*',
            '&',
            '^',
            '%',
            '$',
            '#',
            '@',
            '~',
            '`',
            '|',
            '_',
            '\t',
            '\n'
        ]
        .into_iter()
        .collect();

    pub static ref PUNCTUTATION: HashSet<char> =
        vec![
            ',',
            '.',
            '\'',
            '\"',
            '!',
            '\'',
            ';',
            '?',
            ':',
            ';'
        ]
        .into_iter()
        .collect();

    pub static ref FUNCTIONAL_WORDS: HashSet<&'static str> =
        vec![
            "a",
            "between",
            "in",
            "nor",
            "some",
            "upon",
            "about",
            "both",
            "including",
            "nothing",
            "somebody",
            "us",
            "above",
            "but",
            "inside",
            "of",
            "someone",
            "used",
            "after",
            "by",
            "into",
            "off",
            "something",
            "via",
            "all",
            "can",
            "is",
            "on",
            "such",
            "we",
            "although",
            "cos",
            "it",
            "once",
            "than",
            "what",
            "am",
            "do",
            "its",
            "one",
            "that",
            "whatever",
            "among",
            "down",
            "latter",
            "onto",
            "the",
            "when",
            "an",
            "each",
            "less",
            "opposite",
            "their",
            "where",
            "and",
            "either",
            "like",
            "or",
            "them",
            "whether",
            "another",
            "enough",
            "little",
            "our",
            "these",
            "which",
            "any",
            "every",
            "lots",
            "outside",
            "they",
            "while",
            "anybody",
            "everybody",
            "many",
            "over",
            "this",
            "who",
            "anyone",
            "everyone",
            "me",
            "own",
            "those",
            "whoever",
            "anything",
            "everything",
            "more",
            "past",
            "though",
            "whom",
            "are",
            "few",
            "most",
            "per",
            "through",
            "whose",
            "around",
            "following",
            "much",
            "plenty",
            "till",
            "will",
            "as",
            "for",
            "must",
            "plus",
            "to",
            "with",
            "at",
            "from",
            "my",
            "regarding",
            "toward",
            "within",
            "be",
            "have",
            "near",
            "same",
            "towards",
            "without",
            "because",
            "he",
            "need",
            "several",
            "under",
            "worth",
            "before",
            "her",
            "neither",
            "she",
            "unless",
            "would",
            "behind",
            "him",
            "no",
            "should",
            "unlike",
            "yes",
            "below",
            "i",
            "nobody",
            "since",
            "until",
            "you",
            "beside",
            "if",
            "none",
            "so",
            "up",
            "your",
        ]
            .iter()
            .cloned()
            .collect();
}

