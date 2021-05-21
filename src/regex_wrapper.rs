use std::{
    borrow::Cow,
    hash::{
        Hash,
        Hasher,
    },
};

use regex::Regex;
use serde::{
    de::Error,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};

/// `RegexWrapper` wraps Regex in order for it to be usable in `HashMap`, `LinkedHashMap` etc.
///
/// `RegexWrapper` implements `Eq`, `PartialEq` and `Hash` so it can be used in `HashMap`,
/// `LinkedHashMap` etc `RegexWrapper` implements `Deserialize` and `Serialize` so it can be
/// directly generated via `Contraction`
#[derive(Debug)]
pub struct RegexWrapper(pub Regex);

impl PartialEq for RegexWrapper {
    fn eq(&self, other :&Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for RegexWrapper {}

impl Hash for RegexWrapper {
    fn hash<H :Hasher>(&self, state :&mut H) {
        self.0.as_str().hash(state);
    }
}

impl<'de> Deserialize<'de> for RegexWrapper {
    fn deserialize<D>(d :D) -> Result<Self, D::Error>
    where
        D : Deserializer<'de>,
    {
        let s = <Cow<str>>::deserialize(d)?;

        match s.parse() {
            Ok(regex) => Ok(Self(regex)),
            Err(err) => Err(D::Error::custom(err)),
        }
    }
}

impl Serialize for RegexWrapper {
    fn serialize<S>(&self, serializer :S) -> Result<S::Ok, S::Error>
    where
        S : Serializer,
    {
        self.0.as_str().serialize(serializer)
    }
}
