# ToDo

Write README
fix up this file and the data/expand/README.md

## Functionality

- label the json entries. we might at some point want to introduce entries to contract (rather than expand) contractions!?
- test partials
- add partials via default+add_from_json
- use and test partials
- use testing framework to speed up tests (so we don't have to load everything every time from Contractions::default())
- rename files(CONTRACTIONS_SINGLE_JSON,...) to be more obvious what they are.
- Remove She's, He's, It's
- finish documentation
- upload to crates.io
- generate documentation (and upload it to docs.rs)
- Use parts of speech tagging to figure out if the next word after (She's/...) is a verb, this way we can resolve it
- Use parts-of-speech tagging or named-entity-recognition to figure out if the 's is a possessive or a 'is'
