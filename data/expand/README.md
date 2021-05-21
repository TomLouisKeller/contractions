#


## Source
Original starting point is from:
from https://github.com/kootenpv/contractions

Double contractions are partially from
Robert Charles Lee’s answer on:
https://www.quora.com/Are-double-contractions-like-yourent-allowed-in-English-In-slang-innit-which-is-a-contraction-of-is-not-and-it-isnt-it-is-it-not-is-commonly-used



## Notes

\b in regex is a word boundry

abiguous:
"he’s" => "he is" / "he has"   (as well as all related contractions such as "he’sn’t")
same with "she’s" and "it’s"
"d’y’all" can be "do you all" or "did you all"
"it’sn’t" => "it is not" / "it has not"

rejected:

"o’": "of",
"o’clock": "of the clock",

## contractions_partial

## contractions_single

- Ain’t
  "The word ’ain’t’ is a contraction for am not, is not, are not, has not, and have not in the common English language vernacular. In some dialects ain’t is also used as a contraction of do not, does not, and did not." - https://en.wikipedia.org/wiki/Ain’t
  We therefore do not include ain’t.

- She’s / He’s / It’s
  - "He’s" can be "He is" or "He has".

  - You may only shorten “has” to “’s” when it is part of another verb’s conjugation. So, for example, in “He’s got a child,” the primary verb “got” is in the present perfect, and the “’s” is an auxiliary “has.” Other examples might be “She’s broken the law,” “It’s been a long time,” and “He’s had his chance.”
  There is no ambiguity between “is” and “has” because the surrounding grammar will be different. In the sentence “He’s a child,” there is no other verb, so “has” doesn't work; it has to be “is.” If it were “He’s been a child,” then it expands to “has”—“is” doesn't work in front of “been.”
  https://www.quora.com/In-the-sentence-Hes-a-child-does-Hes-mean-He-is-or-He-has#:~:text=In%20the%20sentence%20as%20written,he%20has%20got%20a%20child).&text=The%20questioner%20asks%20if%20%E2%80%94in,%E2%80%9D%20or%20%E2%80%9Che%20has%E2%80%9D.





## no_apostroph_single_contractions.json

homophones
Some contractions result in actual words if the apostroph is removed.
Those contractions were removed.
"Ill": "I will",
"hell": "he will",
"lets": "let us",
"shell": "she will",
"sos": "so is",
"thiss": "this is",
"tove": "to have",
"were": "we are",
"well": "we will",
"whore": "who are",

## no_apostroph_double_contractions.json

Those are overall pretty silly.

homophones
Some contractions result in actual words if the apostroph is removed.
Those contractions were removed.

"dyall": "do you all",
"howdy": "how do you",
"es": "he is",
"yallll": "you all will",


## Structure

In `find` there is a regular expression which matches all cases in replace.
It’s mainly here, so we don’t have to go through all replace statements, and therefore it improves performance.

The `keys` in `replace` are there to match, the `value` are to replace.
The reason to have multiple in here is to preserve capitalization.
The last entry ought to catch all cases of capitalization and replace it with a all lowercase version.
When replacing, we go from the first entry in `replace` down to the last one.

### Example:
{
    "find": "\\b(?i)i’m(?-i)\\b",
    "replace": {
      "\\bi’m\\b": "i am",
      "\\bI’m\\b": "I am",
      "\\bI’M\\b": "I AM",
      "\\b(?i)i’m(?-i)\\b": "i am"
    }
  },
