//! [`contractions`](https://docs.rs/contractions) is a library to handle contractions
//! So far only data sets to expand contractions are implemented.
//!
//! Expands "I’m" to "I am" etc.
//! The default data set has a replacement for all-lowercase, all-uppercase and first letter
//! uppercase.
//!
//! ## Example
//!
//! ```rust
//! let contractions = contractions::Contractions::default();
//! assert_eq!("I am sure you would have been fine.", contractions.apply("I’m sure you’d’ve been fine."));
//! assert_eq!("Are you sure?", contractions.apply("R u sure?"));
//! ```
//!
//! ```rust
//! let mut contractions = Contractions::new();
//! contractions.add_from_json(contractions::SINGLE_CONTRACTIONS_JSON);
//! assert_eq!("I am sad you couldn’t’ve come.", contractions.apply("I’m sad you couldn’t’ve come."));
//! ```

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
pub const EXPAND_SLANG_JSON :&str = include_str!("../data/expand/slang.json");
/// Contains contractions with one apostroph in json form (eg: I'm, I've, 'twas)
pub const EXPAND_SINGLE_CONTRACTIONS_JSON :&str =
    include_str!("../data/expand/single_contractions.json");
/// Contains contractions with two apostroph in json form (eg: Who'll've, Wouldn't've, Mustn't've)
pub const EXPAND_DOUBLE_CONTRACTIONS_JSON :&str =
    include_str!("../data/expand/double_contractions.json");
/// Contains contractions with three apostroph in json form (Y'all'd've, 'twou'dn't)
pub const EXPAND_TRIPPLE_CONTRACTIONS_JSON :&str =
    include_str!("../data/expand/tripple_contractions.json");
/// Contains most of `CONTRACTIONS_SINGLE_JSON` contractions but without apostroph
pub const EXPAND_SINGLE_NO_APOSTROPHE_CONTRACTIONS_JSON :&str =
    include_str!("../data/expand/single_no_apostroph_contractions.json");
/// Contains most of `CONTRACTIONS_DOUBLE_JSON` contractions but without apostroph
pub const EXPAND_DOUBLE_NO_APOSTROPHE_CONTRACTIONS_JSON :&str =
    include_str!("../data/expand/double_no_apostroph_contractions.json");
/// Contains partial contractions in json form. (eg: 'm, 've, n't, 're)
pub const EXPAND_PARTIAL_CONTRACTIONS_JSON :&str =
    include_str!("../data/expand/partial_contractions.json");

/// The list of all json strings.
///
/// The order is preserved and will be processed from top to bottom.
pub const CONTRACTIONS_JSON_ORDER :&[&str] = &[
    EXPAND_SLANG_JSON,
    EXPAND_DOUBLE_NO_APOSTROPHE_CONTRACTIONS_JSON,
    EXPAND_SINGLE_NO_APOSTROPHE_CONTRACTIONS_JSON,
    EXPAND_TRIPPLE_CONTRACTIONS_JSON,
    EXPAND_DOUBLE_CONTRACTIONS_JSON,
    EXPAND_SINGLE_CONTRACTIONS_JSON,
];

/// [`Contraction`](struct.Contraction.html) holds search term and the replacement-pairs
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

/// Main actor in the [`contractions`](https://docs.rs/contractions) crate
///
/// Stores [`Contractions`](struct.Contractions.html) in a [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
///
/// # Example
/// ```
/// let contractions = contractions::Contractions::default();
/// assert_eq!("I am sure you would have been fine.", contractions.apply("I’m sure you’d’ve been fine."));
/// assert_eq!("Are you sure?", contractions.apply("R u sure?"));
/// ```
#[derive(Debug)]
pub struct Contractions {
    contractions :Vec<Contraction>,
}

impl Default for Contractions {
    /// Returns the built in configuration for [`Contractions`](struct.Contractions.html)
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
    /// Creates empty [`Contractions`](struct.Contractions.html)
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

    /// Deserialize `Contraction` from json
    ///
    /// Convenience method that chains [`Contractions::new()`](struct.Contractions.html#method.new)
    /// and [`Contractions::add_from_json()`](struct.Contractions.html#method.add_from_json)
    ///
    /// # Example
    /// ```
    /// use contractions::{self, Contractions};
    /// let contractions = Contractions::from_json(&[contractions::SINGLE_CONTRACTIONS_JSON, contractions::SINGLE_NO_APOSTROPHE_CONTRACTIONS_JSON]);
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

    /// Add `Contraction`s from a json string to an existing
    /// [`Contractions`](struct.Contractions.html) struct
    ///
    /// # Example
    /// ```
    /// use contractions::{self, Contractions};
    /// let mut contractions = Contractions::new();
    /// contractions.add_from_json(contractions::SINGLE_CONTRACTIONS_JSON);
    /// ```
    ///
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

    /// Remove a `Contraction` from [`Contractions`](struct.Contractions.html)
    ///
    /// Provide the exact `find` key to delete the corresponding `Contraction`
    ///
    /// # Example
    /// ```
    /// use contractions::{self, Contractions};
    /// let mut contractions = Contractions::new();
    /// assert_eq!("I’m happy", contractions.apply("I’m happy"));
    /// contractions.add_from_json(contractions::SINGLE_CONTRACTIONS_JSON);
    /// assert_eq!("I am happy", contractions.apply("I’m happy"));
    /// contractions.remove("\\b(?i)i['’`]m(?-i)\\b");
    /// assert_eq!("I’m happy", contractions.apply("I’m happy"));
    /// ```
    pub fn remove(&mut self, key :&str) {
        self.contractions.retain(|c| c.find.as_str() != key);
    }

    /// Add a contraction to [`Contractions`](struct.Contractions.html)
    ///
    /// # Example
    /// ```
    /// use contractions::{self, Contractions};
    /// let mut contractions = Contractions::new();
    /// assert_eq!("I’m happy", contractions.apply("I’m happy"));
    /// let find = r#"\b(?i)i['’`]m(?-i)\b"#;
    /// let mut replace = linked_hash_map::LinkedHashMap::new();
    /// replace.insert(r#"\bi['’`]m\b"#, "i am");
    /// replace.insert(r#"\bI['’`]m\b"#, "I am");
    /// replace.insert(r#"\bI['’`]M\b"#, "I AM");
    /// contractions.add(find, replace);
    /// assert_eq!("I am happy", contractions.apply("I’m happy"));
    /// ```
    ///
    /// # Errors
    /// Returns an Error if `find` or the key in the `replace`
    /// cannot be successfully turned into a Regex
    pub fn add(
        &mut self,
        find :&str,
        replace :LinkedHashMap<&str, &str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let find = Regex::new(find)?;
        let in_replace = replace;
        let mut replace :LinkedHashMap<RegexWrapper, String> = LinkedHashMap::new();
        for (f, r) in in_replace {
            replace.insert(RegexWrapper(Regex::new(f)?), r.to_string());
        }

        let contraction = Contraction { find, replace };
        self.contractions.push(contraction);
        Ok(())
    }

    /// Replace contractions with their long form
    ///
    /// # Example
    /// ```
    /// use contractions::Contractions;
    /// let contractions = Contractions::default();
    /// assert_eq!("I am your brother’s son", contractions.apply("I’m your brother’s son"));
    /// ```
    #[must_use]
    pub fn apply(&self, input :&str) -> String {
        let mut output = input.to_string();
        for contraction in &self.contractions {
            if contraction.is_match(&output) {
                contraction.replace_all(&mut output);
            }
        }
        output
    }
}
