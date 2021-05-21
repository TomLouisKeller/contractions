# ToDo

Write README
fix up this file and the data/expand/README.md

## Functionality

- use Cow in expand()
- use &mut String as input in replace_all (or Cow)
- in from_json we could find a way to deserialize the strings that work, and simply log those who don't. - think about it
- Assure both apostroph as well as grave accent work ( ' ’ `)
- Don't replace when a contraction is part of a larger term. eg "Adelphe's" currently turn's to "Adelphe is" because of "he's" => "he is"
- Use parts-of-speech tagging or named-entity-recognition to figure out if the 's is a possessive or a 'is'
- combine "daren't" "daresn't" "dasn't"  to one regex
- combine shan't and shalln't in one regex
- What about months?
- use partials again
- move current contractions json files into a "expand" folder
- Stabilize api
- add() and remove() to change dataset
- make json dataset properly accessible to library users
- finish documentation
- upload to crates.io
- generate documentation (and upload it to docs.rs)

## Contractions

- "He's" - In most dialects, he’s as a contraction of he has is only used to mark the perfect tense (“He’s done something.”, “He has done something.”), and not to signify possession (“He has something.”). Some dialects, however, use he’s for both.


## Test

- Use a framework to turn the tests into one with a single setup `for Contractions::default()`
