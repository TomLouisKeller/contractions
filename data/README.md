#


## Source
Original starting point is from:
from https://github.com/kootenpv/contractions

Double contractions are partially from
Robert Charles Lee's answer on:
https://www.quora.com/Are-double-contractions-like-yourent-allowed-in-English-In-slang-innit-which-is-a-contraction-of-is-not-and-it-isnt-it-is-it-not-is-commonly-used



## Notes

\b in regex is a word boundry

abiguous:
"he's" => "he is" / "he has"   (as well as all related contractions such as "he’sn’t")
same with "she's" and "it's"
"d’y’all" can be "do you all" or "did you all"
"it’sn’t" => "it is not" / "it has not"

rejected:

"o'": "of",
"o'clock": "of the clock",

## contractions_partial

## contractions_single
"The word 'ain't' is a contraction for am not, is not, are not, has not, and have not in the common English language vernacular. In some dialects ain't is also used as a contraction of do not, does not, and did not." - https://en.wikipedia.org/wiki/Ain't
We therefore do not include ain't.

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
It's mainly here, so we don't have to go through all replace statements, and therefore it improves performance.

The `keys` in `replace` are there to match, the `value` are to replace.
The reason to have multiple in here is to preserve capitalization.
The last entry ought to catch all cases of capitalization and replace it with a all lowercase version.
When replacing, we go from the first entry in `replace` down to the last one.

### Example:
{
    "find": "\\b(?i)i'm(?-i)\\b",
    "replace": {
      "\\bi'm\\b": "i am",
      "\\bI'm\\b": "I am",
      "\\bI'M\\b": "I AM",
      "\\b(?i)i'm(?-i)\\b": "i am"
    }
  },
