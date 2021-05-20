
# Contractions

[![MIT licensed][mit-badge]][mit-url]

---

**Notice**: `Contractions`' API is not stabilized yet and still work in progress

---

`Contractions` is a rust library to expand contractions in English.  
So far contractions can only be expanded.

Expands "I'm" to "I am" etc.
Preserves capitalization (as regular expressions for all-lowercase, all-uppercase and first letter uppercase)

## Problem cases:
- Ain't
    "The word 'ain't' is a contraction for am not, is not, are not, has not, and have not in the common English language vernacular. In some dialects ain't is also used as a contraction of do not, does not, and did not." - https://en.wikipedia.org/wiki/Ain't  
    "Ain't" is not replaced!
- Some contractions with "'s" can be is or a possesive
  - "everyone's" is replaced with "Everyone is"
  - "somebody's" is replaced with "somebody is"
  - "someone's" is replaced with "someone is"
  - "Carl's" is not replaced!
- He's -> can be he is or he has. We replace it with "He is".

---

This project is licensed under the terms of the MIT license.

[mit-url]: LICENSE
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
