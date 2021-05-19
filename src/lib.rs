use linked_hash_map::LinkedHashMap;
use regex::Regex;

use std::error::Error;

mod regex_wrapper;
use regex_wrapper::RegexWrapper;

use serde::{Deserialize, Serialize};

pub const CONTRACTIONS_PARTIAL_JSON: &str =
    include_str!("../data/expand/contractions_partial.json");
pub const CONTRACTIONS_SINGLE_JSON: &str = include_str!("../data/expand/contractions_single.json");
pub const CONTRACTIONS_DOUBLE_JSON: &str = include_str!("../data/expand/contractions_double.json");
pub const CONTRACTIONS_TRIPPLE_JSON: &str =
    include_str!("../data/expand/contractions_tripple.json");
pub const CONTRACTIONS_SINGLE_NO_APOSTROPHE_JSON: &str =
    include_str!("../data/expand/contractions_single_no_apostroph.json");
pub const CONTRACTIONS_DOUBLE_NO_APOSTROPHE_JSON: &str =
    include_str!("../data/expand/contractions_double_no_apostroph.json");
pub const SLANG_JSON: &str = include_str!("../data/expand/slang.json");

pub const CONTRACTIONS_JSON_ORDER: &[&str] = &[
    SLANG_JSON,
    CONTRACTIONS_DOUBLE_NO_APOSTROPHE_JSON,
    CONTRACTIONS_SINGLE_NO_APOSTROPHE_JSON,
    CONTRACTIONS_TRIPPLE_JSON,
    CONTRACTIONS_DOUBLE_JSON,
    CONTRACTIONS_SINGLE_JSON,
    // CONTRACTIONS_PARTIAL_JSON,
];

/// Contraction holds search term and
#[derive(Debug, Serialize, Deserialize)]
pub struct Contraction {
    #[serde(with = "serde_regex")]
    find: Regex,
    // #[serde(with = "serde_regex_wrapper")]
    replace: LinkedHashMap<RegexWrapper, String>,
}

impl Contraction {
    pub fn is_match(&self, text: &str) -> bool {
        self.find.is_match(text)
    }

    pub fn replace_all(&self, text: &str) -> String {
        let mut output = text.to_string();
        for (search, replace) in self.replace.iter() {
            output = search.0.replace_all(&output, replace).into_owned();
        }
        output
    }
}

pub struct Contractions {
    contractions: Vec<Contraction>,
}

impl Contractions {
    pub fn default() -> Result<Self, Box<dyn Error>> {
        Ok(Self::from_json(CONTRACTIONS_JSON_ORDER)?)
    }

    // TODO: Serialize and deserialize Contractions, so we simply have to push in the contractions into the holder
    /// Deserialize quoter from json
    pub fn from_json(contractions_as_str: &[&str]) -> Result<Self, Box<dyn Error>> {
        let mut contractions: Vec<Contraction> = Vec::new();
        for s in contractions_as_str {
            // println!("s: {}", s);
            let mut contr_part: Vec<Contraction> = serde_json::from_str(s).unwrap();
            contractions.append(&mut contr_part);
            // for (in_find, in_replace) in contr_part.iter() {
            //     let find = Regex::new(&in_find).unwrap();
            //     let find = RegexWrapper(find);

            //     let mut replace: LinkedHashMap<RegexWrapper, String> = LinkedHashMap::new();
            //     for (repl_regex, repl_replace) in in_replace.iter() {
            //         let repl_regex = Regex::new(&repl_regex).unwrap();
            //         let repl_regex = RegexWrapper(repl_regex);
            //         replace.insert(repl_regex, repl_replace.to_string());
            //     }

            //     contractions.push(Contraction { find, replace });
            // }
        }

        Ok(Contractions { contractions })
    }

    pub fn list(&self) {
        for contr in self.contractions.iter() {
            println!("{:?}", contr);
        }
    }

    // Returns a reference to the value that `key` maps to.
    // pub fn get(&self, key :&str) -> Option<&str> {
    //     self.contractions.get(key)
    // }

    /// Replace contractions with their long form
    ///
    /// # Example
    /// '''
    /// assert_eq("I am your brother's son", replace("I'm your brother's son"));
    /// '''
    pub fn expand(&self, input: &str) -> String {
        let mut output = input.to_string();

        for contraction in self.contractions.iter() {
            // output = output.replace(short, long);
            // println!("{}", regex.0);
            if contraction.is_match(&output) {
                println!("Found match {:?}", contraction.find.as_str());
                // output = regex.0.replace_all(&output, long).into_owned();
                output = contraction.replace_all(&output);
            }
        }

        output
    }
}

#[cfg(test)]
#[path = "./unit_tests/ut_contractions.rs"]
mod unit_tests;
