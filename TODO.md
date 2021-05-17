# ToDo

## Functionality

- Assure both apostroph as well as grave accent work ( ' ’ `)
- Ignore case when searching,
- Respect original capitalization(case-sensitivity) when replacing
- Don't replace when a contraction is part of a larger term. eg "Adelphe's" currently turn's to "Adelphe is" because of "he's" => "he is"
- Use parts-of-speech tagging or named-entity-recognition to figure out if the 's is a possessive or a 'is'

## Contractions

- "He's" - In most dialects, he’s as a contraction of he has is only used to mark the perfect tense (“He’s done something.”, “He has done something.”), and not to signify possession (“He has something.”). Some dialects, however, use he’s for both.
- Some contractions with "'s" can be is or a possisive
  - "everyone's" can be "everyone's country" or "everyone is"
  - "somebody's"

## Test

- Use a framework to turn the tests into one with a single setup `for Contractions::default()`


## Performance

- Benchmark std::collections::BTreeMap vs LinkedHashMap
