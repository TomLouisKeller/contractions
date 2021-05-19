# ToDo

## Functionality

- Assure both apostroph as well as grave accent work ( ' ’ `)
- Don't replace when a contraction is part of a larger term. eg "Adelphe's" currently turn's to "Adelphe is" because of "he's" => "he is"
- Use parts-of-speech tagging or named-entity-recognition to figure out if the 's is a possessive or a 'is'
- combine "daren't" "daresn't" "dasn't"  to one regex
- What about months?
- use partials again

## Contractions

- "He's" - In most dialects, he’s as a contraction of he has is only used to mark the perfect tense (“He’s done something.”, “He has done something.”), and not to signify possession (“He has something.”). Some dialects, however, use he’s for both.


## Test

- Use a framework to turn the tests into one with a single setup `for Contractions::default()`
