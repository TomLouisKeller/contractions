
# Contractions

[![MIT licensed][mit-badge]][mit-url]

---

**Notice**: `Contractions`’ API is not stabilized yet and still work in progress

---

`Contractions` is a rust library to expand contractions in English.  
So far contractions can only be expanded.

Expands "I’m" to "I am" etc.
The default data set has a replacement for all-lowercase, all-uppercase and first letter uppercase.

## Problem cases (default data set):
- Ain’t
  "The word ’ain’t’ is a contraction for am not, is not, are not, has not, and have not in the common English language vernacular. In some dialects ain’t is also used as a contraction of do not, does not, and did not." - https://en.wikipedia.org/wiki/Ain’t  
  - The default dataset replaces does not replace "Ain’t"
- Some contractions with "’s" can be "is" or a possessive
  - The default dataset replaces "Everyone’s" => "Everyone is"
  - The default dataset replaces "Somebody’s" => "Somebody is"
  - The default dataset replaces "Someone’s" => "Someone is"
  - The default dataset replaces does not replace any other contractions with ’s such as "Carl’s"
- She’s / He’s / It’s
  - "He’s" can be "He is" or "He has".
  - The default dataset replaces "He’s" => "He is", "She’s" => "She is", "It’s" => "It is"

[mit-url]: LICENSE
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
