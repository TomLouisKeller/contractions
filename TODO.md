# ToDo

## Functionality

- Assure both apostroph as well as grave accent work
- Ignore case when searching, but replace original capitalization
- Don't replace when a contraction is part of a larger term. eg "Adelphe's" currently turn's to "Adelphe is" because of "he's" => "he is"

## Test

- Use a framework to turn the tests into one with a single setup `for Contractions::default()`


## Performance

- Benchmark std::collections::BTreeMap vs LinkedHashMap
