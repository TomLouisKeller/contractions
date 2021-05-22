# ToDo

Write README
fix up this file and the data/expand/README.md

## Functionality

- use testing framework to speed up tests (so we don't have to load everything every time from Contractions::default())
- rename files(CONTRACTIONS_SINGLE_JSON,...) to be more obvious what they are.
- only enter one string to add_from_json ??? or array? does it matter?
- test partials
- add partials via default+add_from_json
- add() and remove() to change dataset
- Remove She's, He's, It's
- use and test partials
- finish documentation
- upload to crates.io
- generate documentation (and upload it to docs.rs)
- Use parts of speech tagging to figure out if the next word after (She's/...) is a verb, this way we can resolve it
- Use parts-of-speech tagging or named-entity-recognition to figure out if the 's is a possessive or a 'is'
