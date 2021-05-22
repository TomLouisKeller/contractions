//! `Contractions` is a library to handle contractions
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]

#[macro_use]
extern crate log;

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

    fn replace_all(&self, text :&mut String) {
        debug!("Replace all - Pattern: \"{}\"", &self.find);
        for (search, replace) in self.replace.iter() {
            *text = search.0.replace_all(text, replace).into_owned();
        }
    }
}

/// Main actor in the `contractions` crate
///
/// Stores `Contraction` in a `Vec`
///
/// # Example
/// ```
/// let contractions = contractions::Contractions::default();
/// assert_eq!("I am sure you would have been fine.", contractions.expand("I’m sure you’d’ve been fine."));
/// assert_eq!("Are you sure?", contractions.expand("R u sure?"));
/// ```
pub struct Contractions {
    contractions :Vec<Contraction>,
}

impl Default for Contractions {
    /// Returns the built in configuration for `Contractions`
    ///
    /// # Example
    /// ```
    /// use contractions::Contractions;
    /// let contractions = Contractions::new();
    /// ```
    /// # Panics
    /// Only panics when the library internal configuration is faulty
    /// this ought to only happen during development
    fn default() -> Self {
        Self::from_json(CONTRACTIONS_JSON_ORDER).unwrap()
    }
}

impl Contractions {
    /// Creates empty `Contractions`
    ///
    /// # Example
    /// ```
    /// use contractions::{self, Contractions};
    /// let contractions = Contractions::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            contractions :vec![],
        }
    }

    /// Deserialize quoter from json
    ///
    /// Convenience method for `Contractions::new()` `Contractions::add_from_json()`
    ///
    /// # Example
    /// ```
    /// use contractions::{self, Contractions};
    /// let contractions = Contractions::from_json(&[contractions::CONTRACTIONS_SINGLE_JSON, contractions::CONTRACTIONS_SINGLE_NO_APOSTROPHE_JSON]);
    /// ```
    /// # Errors
    /// Returns an Error if deserialization fails
    pub fn from_json(contractions_as_str :&[&str]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut contractions = Self::new();
        for s in contractions_as_str {
            contractions.add_from_json(s)?;
        }
        Ok(contractions)
    }

    /// Add contractions from a json string to an existing `Contractions` struct
    ///
    /// # Example
    /// ```
    /// use contractions::{self, Contractions};
    /// let mut contractions = Contractions::new();
    /// contractions.add_from_json(contractions::CONTRACTIONS_SINGLE_JSON);
    /// ```
    /// # Errors
    /// Returns an Error if deserialization fails
    pub fn add_from_json(
        &mut self,
        contractions_as_str :&str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut contr_part :Vec<Contraction> = serde_json::from_str(contractions_as_str)?;
        debug!("Added contractions from json.\n{:#?}\n", contr_part);
        self.contractions.append(&mut contr_part);
        Ok(())
    }

    /// Remove a contraction from `Contractions`
    ///
    /// # Example
    /// ```
    /// use contractions::{self, Contractions};
    /// let mut contractions = Contractions::new();
    /// assert_eq!("I’m happy", contractions.expand("I’m happy"));
    /// contractions.add_from_json(contractions::CONTRACTIONS_SINGLE_JSON);
    /// assert_eq!("I am happy", contractions.expand("I’m happy"));
    /// contractions.remove("\\b(?i)i['’`]m(?-i)\\b");
    /// assert_eq!("I’m happy", contractions.expand("I’m happy"));
    /// ```
    pub fn remove(&mut self, key :&str) {
        self.contractions.retain(|c| c.find.as_str() != key);
    }

    /// Replace contractions with their long form
    ///
    /// # Example
    /// ```
    /// use contractions::Contractions;
    /// let contractions = Contractions::default();
    /// assert_eq!("I am your brother’s son", contractions.expand("I’m your brother’s son"));
    /// ```
    #[must_use]
    pub fn expand(&self, input :&str) -> String {
        let mut output = input.to_string();
        for contraction in &self.contractions {
            if contraction.is_match(&output) {
                contraction.replace_all(&mut output);
            }
        }
        output
    }
}
