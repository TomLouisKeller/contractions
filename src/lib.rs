//! `Contractions` is a library to handle contractions
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]

#[macro_use]
extern crate log;

use std::error::Error;

use linked_hash_map::LinkedHashMap;
use regex::Regex;

mod regex_wrapper;
use regex_wrapper::RegexWrapper;
use serde::{
    Deserialize,
    Serialize,
};

/// Contains slang terms which will be expanded/changed to their full form
pub const SLANG_JSON :&str = include_str!("../data/expand/slang.json");
/// Contains contractions with one apostroph in json form (eg: I'm, I've, 'twas)
pub const CONTRACTIONS_SINGLE_JSON :&str = include_str!("../data/expand/contractions_single.json");
/// Contains contractions with two apostroph in json form (eg: Who'll've, Wouldn't've, Mustn't've)
pub const CONTRACTIONS_DOUBLE_JSON :&str = include_str!("../data/expand/contractions_double.json");
/// Contains contractions with three apostroph in json form (Y'all'd've, 'twou'dn't)
pub const CONTRACTIONS_TRIPPLE_JSON :&str =
    include_str!("../data/expand/contractions_tripple.json");
/// Contains most of `CONTRACTIONS_SINGLE_JSON` contractions but without apostroph
pub const CONTRACTIONS_SINGLE_NO_APOSTROPHE_JSON :&str =
    include_str!("../data/expand/contractions_single_no_apostroph.json");
/// Contains most of `CONTRACTIONS_DOUBLE_JSON` contractions but without apostroph
pub const CONTRACTIONS_DOUBLE_NO_APOSTROPHE_JSON :&str =
    include_str!("../data/expand/contractions_double_no_apostroph.json");
/// Contains partial contractions in json form. (eg: 'm, 've, n't, 're)
pub const CONTRACTIONS_PARTIAL_JSON :&str =
    include_str!("../data/expand/contractions_partial.json");

/// The list of all json strings.
///
/// The order used to matter, but does no longer.
/// The order is preserved and will be processed from top to bottom.
pub const CONTRACTIONS_JSON_ORDER :&[&str] = &[
    SLANG_JSON,
    CONTRACTIONS_DOUBLE_NO_APOSTROPHE_JSON,
    CONTRACTIONS_SINGLE_NO_APOSTROPHE_JSON,
    CONTRACTIONS_TRIPPLE_JSON,
    CONTRACTIONS_DOUBLE_JSON,
    CONTRACTIONS_SINGLE_JSON,
];

/// Contraction holds search term and the replacement-pairs
#[derive(Debug, Serialize, Deserialize)]
struct Contraction {
    #[serde(with = "serde_regex")]
    find :Regex,
    replace :LinkedHashMap<RegexWrapper, String>,
}

impl Contraction {
    fn is_match(&self, text :&str) -> bool {
        if self.find.is_match(text) {
            debug!(
                "Found match - Pattern: \"{}\" - Text: \"{}\"",
                &self.find, &text
            );
            true
        } else {
            false
        }
    }

    fn replace_all(&self, text :&str) -> String {
        debug!("Replace all - Pattern: \"{}\"", &self.find);
        let mut output = text.to_string();
        for (search, replace) in self.replace.iter() {
            output = search.0.replace_all(&output, replace).into_owned();
        }
        output
    }
}

/// Main actor in the `contractions` crate
///
/// Stores `Contraction` in a `Vec`
///
/// # Example
/// ```
/// let contractions = contractions::Contractions::default();
/// assert_eq!("I am sure you would have been fine.", contractions.expand("I'm sure you'd've been fine."));
/// assert_eq!("Are you sure?", contractions.expand("R u sure?"));
/// ```
pub struct Contractions {
    contractions :Vec<Contraction>,
}

impl Default for Contractions {
    /// Returns the built in configuration for `Contractions`
    ///
    /// # Panics
    /// Only panics when the library internal configuration is faulty
    /// this ought to only happen during development
    fn default() -> Self {
        Self::from_json(CONTRACTIONS_JSON_ORDER).unwrap()
    }
}

impl Contractions {
    // TODO: Serialize and deserialize Contractions, so we simply have to push in the contractions
    // into the holder
    /// Deserialize quoter from json
    ///
    /// # Errors
    /// Returns an Error if deserialization fails
    pub fn from_json(contractions_as_str :&[&str]) -> Result<Self, Box<dyn Error>> {
        let mut contractions :Vec<Contraction> = Vec::new();
        for s in contractions_as_str {
            let mut contr_part :Vec<Contraction> = serde_json::from_str(s)?;
            contractions.append(&mut contr_part);
        }
        debug!("Loaded contractions from json.\n{:#?}\n", contractions);
        Ok(Self { contractions })
    }

    /// Replace contractions with their long form
    ///
    /// # Example
    /// ```
    /// let contractions = contractions::Contractions::default();
    /// assert_eq!("I am your brother's son", contractions.expand("I'm your brother's son"));
    /// ```
    #[must_use]
    pub fn expand(&self, input :&str) -> String {
        let mut output = input.to_string();
        for contraction in &self.contractions {
            if contraction.is_match(&output) {
                output = contraction.replace_all(&output);
            }
        }
        output
    }
}

#[cfg(test)]
#[path = "./unit_tests/ut_contractions.rs"]
mod unit_tests;
