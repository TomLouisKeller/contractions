use linked_hash_map::LinkedHashMap;
use regex::Regex;
use std::error::Error;
use std::hash::{Hash, Hasher};

// const CONTRACTIONS_PARTIAL_JSON: &str = include_str!("../data/contractions_partial.json");
const CONTRACTIONS_SINGLE_JSON: &str = include_str!("../data/contractions_single.json");
const CONTRACTIONS_DOUBLE_JSON: &str = include_str!("../data/contractions_double.json");
const CONTRACTIONS_TRIPPLE_JSON: &str = include_str!("../data/contractions_tripple.json");
const CONTRACTIONS_SINGLE_NO_APOSTROPHE_JSON: &str =
    include_str!("../data/contractions_single_no_apostroph.json");
const CONTRACTIONS_DOUBLE_NO_APOSTROPHE_JSON: &str =
    include_str!("../data/contractions_double_no_apostroph.json");
const SLANG_JSON: &str = include_str!("../data/slang.json");

const CONTRACTIONS_JSON_ORDER: &[&str] = &[
    SLANG_JSON,
    CONTRACTIONS_DOUBLE_NO_APOSTROPHE_JSON,
    CONTRACTIONS_SINGLE_NO_APOSTROPHE_JSON,
    CONTRACTIONS_TRIPPLE_JSON,
    CONTRACTIONS_DOUBLE_JSON,
    CONTRACTIONS_SINGLE_JSON,
    // CONTRACTIONS_PARTIAL_JSON,
];

#[derive(Debug)]
struct RegexWrapper(Regex);

impl PartialEq for RegexWrapper {
    fn eq(&self, other: &RegexWrapper) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for RegexWrapper {}

impl Hash for RegexWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state);
    }
}

pub struct Contractions {
    contractions: LinkedHashMap<RegexWrapper, String>,
}

impl Contractions {
    pub fn default() -> Result<Self, Box<dyn Error>> {
        Ok(Self::from_json(CONTRACTIONS_JSON_ORDER)?)
    }

    // TODO: error handling
    // TODO: check if there are quotes. if not => log error and
    // make sure Quoter can return None or some solution like that
    /// Deserialize quoter from json
    pub fn from_json(contractions_as_str: &[&str]) -> Result<Self, Box<dyn Error>> {
        let mut contractions: LinkedHashMap<RegexWrapper, String> = LinkedHashMap::new();

        for s in contractions_as_str {
            // println!("s: {}", s);
            let contr_part: LinkedHashMap<String, String> = serde_json::from_str(s)?;

            // LinkedHashMap doesn't have append, so we have to add one entry at a time
            for (e_short, e_long) in contr_part.iter() {
                let e_short = format!(r"\b(?i){}(?-i)\b", e_short);
                let regex = Regex::new(&e_short).unwrap();
                // println!("{}", regex);
                contractions.insert(RegexWrapper(regex), e_long.to_string());
            }
        }

        Ok(Contractions { contractions })
    }

    pub fn list(&self) {
        for (short, long) in self.contractions.iter() {
            println!("{:?} -> {}", short, long);
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

        for (regex, long) in self.contractions.iter() {
            // output = output.replace(short, long);
            // println!("{}", regex.0);
            if regex.0.is_match(&output) {
                println!("Found match {:?}", regex.0);
                output = regex.0.replace_all(&output, long).into_owned();
            }
        }

        output
    }
}

// for word in input.split(' ') {
//     match self.contractions.get(word) {
//         Some(replacement) => output.push_str(replacement),
//         None => output.push_str(word),
//     }
//     output.push(' ');
// }

// remove last space
// output.pop();

// struct Contraction {
//     short_form :String,
//     long_form :String,
// }

#[cfg(test)]
#[path = "./unit_tests/ut_contractions.rs"]
mod unit_tests;
