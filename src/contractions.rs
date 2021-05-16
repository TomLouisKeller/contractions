use std::collections::HashMap;

use serde::Deserialize;

const CONTRACTIONS_DATA :&str = include_str!("../data/contractions.json5");

#[derive(Deserialize, Debug)]
pub struct Contractions {
    contractions :HashMap<String, String>,
}

impl Default for Contractions {
    fn default() -> Self {
        Self::from_json(CONTRACTIONS_DATA)
    }
}

impl Contractions {
    // TODO: error handling
    // TODO: check if there are quotes. if not => log error and
    // make sure Quoter can return None or some solution like that
    /// Deserialize quoter from json
    fn from_json(contractions_as_str :&str) -> Self {
        json5::from_str(contractions_as_str).unwrap()
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
    pub fn expand(&self, input :&str) -> String {
        // TODO: what about periods, etc
        let mut output = String::with_capacity(input.len());
        for word in input.split(' ') {
            match self.contractions.get(word) {
                Some(replacement) => output.push_str(replacement),
                None => output.push_str(word),
            }
            output.push(' ');
        }

        // remove last space
        output.pop();

        output
    }
}

// struct Contraction {
//     short_form :String,
//     long_form :String,
// }

#[cfg(test)]
#[path = "./unit_tests/ut_contractions.rs"]
mod unit_tests;
