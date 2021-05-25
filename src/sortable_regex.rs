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

/// [`RegexWrapper`](struct.RegexWrapper.html) wraps Regex in order for it to be usable in
/// [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), [`LinkedHashMap`](https://docs.rs/linked-hash-map/0.5.4/linked_hash_map/) etc.
///
/// [`RegexWrapper`](struct.RegexWrapper.html) implements [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html), [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) and [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) so it can be used in [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
/// [`LinkedHashMap`](https://docs.rs/linked-hash-map/0.5.4/linked_hash_map/) etc [`RegexWrapper`](struct.RegexWrapper.html) implements [`Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html) and [`Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) so it can be
/// directly generated via [`Contraction`](struct.Contraction.html)
#[derive(Debug)]
pub struct SortableRegex(pub Regex);

impl PartialEq for SortableRegex {
    fn eq(&self, other :&Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for SortableRegex {}

impl Hash for SortableRegex {
    fn hash<H :Hasher>(&self, state :&mut H) {
        self.0.as_str().hash(state);
    }
}

impl<'de> Deserialize<'de> for SortableRegex {
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

impl Serialize for SortableRegex {
    fn serialize<S>(&self, serializer :S) -> Result<S::Ok, S::Error>
    where
        S : Serializer,
    {
        self.0.as_str().serialize(serializer)
    }
}
