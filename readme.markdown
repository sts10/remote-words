# Remote Words

Passphrase word lists optimized to reduce clicks of a TV remote when entering a passphrase, including travel distance between letters.

The number of "clicks" a given word requires is equal to its length plus the number of navigation clicks it takes to get from each letter to the next. This second measurement depends greatly on the **layout** of letters that the app/device presents to the user. To that end, this repo has a number of word lists in `lists/` sub-directory, each based on a different layout. The layouts are summarized below.

This project (the Rust code and the resulting word lists) is mostly a **proof of concept** to show how very specific word lists can aid users.

## Example

For example, given this keyboard layout (which I call simply `Qwerty`):
```
qwertyuiop
asdfghjkl
zxcvbnm
```

"wax" costs 7 clicks (1 per letter, plus 4 to navigate from the 'w' to the 'a' to the 'x'), whereas "lap" costs 21 clicks, since you've got to cross the keyboard twice. Thus, for user convenience, a word list made from this particular keyboard layout should prioritize "wax" over "lap", despite them being the same length.

On a keyboard layout that's just all the letters in one row alphabetically (`abcdefghijklmnopqrstuvwxyz`), which I refer to as `AlphaLine`, the costs change: "wax" costs 48 clicks while "lap" only costs 29 clicks.

## Assumptions

This program assumes that if the user is, say, on the 'w', they can't move left twice to reach the 'p'. That'd be a neat optional feature though if you want to write a PR!

## Usage (How to generate word lists yourself)

Lists should be available in the `lists/` directory, but the Rust code that I used to generate these lists from a "raw" list is also available. Here's how to run that code (and overwrite the lists in `lists/`):

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `mkdir lists`
3. `cargo run --release`

Locations of input file and created files are all hard-coded. Created lists will be printed in to `lists/`.

## Lists/layouts

Each list in the `lists/` sub-directory corresponds to a different, common keyboard layout. All lists are 7,776-words long, which means that each word adds approximately 12.925 bits to a passphrase.

**Guidance**: to minimize the number of "clicks" you must execute to enter a passphrase, use the word list that corresponds to the keyboard layout you're asked to enter the passphrase with.

All layouts are ortholinear, which is both realistic (from the layouts I've seen on Smart TV apps) and simplifies travel-distance calculations.

### qwerty.txt

```txt
qwertyuiop
asdfghjkl
zxcvbnm
```

### alpha-square.txt
```txt
abcdef
ghijkl
mnopqr
stuvwx
yz
```

### alpha-line.txt

Just what it sounds like (think some Apple TVs use this layout?).

```txt
abcdefghijklmnopqrstuvwxyz
```

## Disclaimer: Profane words

These lists may have profane, indecent or otherwise objectionable words in them.

## Where the words come from

The `raw.txt` word list -- which is used by the Rust code to create the shorter, optimized lists -- came from Google Books Ngram word frequency data (and since edited). See below for more information.

## More word lists

You can find additional [word lists I've created here](https://github.com/sts10/generated-wordlists).

## On licensing/usage

This project and its word lists are licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/). See LICENSE file for more information.

The words for some of these generated word lists come from Google Books Ngram data. This data compilation "is licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/)" ([source](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html)). This project has no association with Google, nor does Google endorse this project. [More information available at the original project's repo](https://github.com/sts10/common_word_list_maker).
