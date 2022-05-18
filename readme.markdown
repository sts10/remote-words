# Remote Words

Passphrase word lists optimized to reduce clicks of a TV remote when entering a passphrase, including travel distance between letters.

The number of "clicks" a given word requires is equal to its length plus the number of navigation clicks it takes to get from each letter to the next. This second measurement depends greatly on the **layout** of letters that the app/device presents to the user. To that end, this repo has a number of word lists in `lists/` sub-directory, each based on a different layout. The layouts are summarized below.


## Usage (for generating word lists)

1. Install Rust
2. `mkdir lists`
3. `cargo run --release`

Created lists will be printed in to `lists/`.

## Lists/layouts

Each list in the `lists/` sub-directory corresponds to a different, common keyboard layout.

**Guidance**: to minimize the number of "clicks" you must execute to enter a passphrase, use the word list that corresponds to the keyboard layout you're asked to enter the passphrase with.

All layouts are orthogonal, which is both realistic (from the layouts I've seen on Smart TV apps) and simplifies travel-distance calculations.

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

## Entropy per word

Each list has 18,231 words, which means that each word of a passphrases adds 14.154 bits of entropy to a passphrase.


## Where the words come from

The `raw.txt` word list -- which is used by the Rust code to create the shorter, optimized lists -- came from Google Books Ngram word frequency data (and since edited). See below for more information.

## More word lists

You can find additional [word lists I've created here](https://github.com/sts10/generated-wordlists).

## On licensing/usage

This project and its word lists are licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/). See LICENSE file for more information.

The words for some of these generated word lists come from Google Books Ngram data. This data compilation "is licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/)" ([source](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html)). This project has no association with Google, nor does Google endorse this project. [More information available at the original project's repo](https://github.com/sts10/common_word_list_maker).
