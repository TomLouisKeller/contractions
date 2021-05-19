use regex::Regex;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::hash::{Hash, Hasher};

/// RegexWrapper is here so we can use Regex in HashMap, LinkedHashMap etc.
///
/// RegexWrapper implements `Eq`, `PartialEq` and `Hash` so we can use it in Maps.
#[derive(Debug)]
pub struct RegexWrapper(pub Regex);

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

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D>(d: D) -> Result<RegexWrapper, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <Cow<str>>::deserialize(d)?;

        match s.parse() {
            Ok(regex) => Ok(RegexWrapper(regex)),
            Err(err) => Err(D::Error::custom(err)),
        }
    }
}

// impl<'a> Serialize for &'a RegexWrapper {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         self.0.as_str().serialize(serializer)
//     }
// }

impl Serialize for RegexWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.as_str().serialize(serializer)
    }
}
