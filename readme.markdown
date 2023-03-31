# Remote Words

Passphrase word lists optimized to reduce clicks of a TV remote (or video game console controller) when entering a passphrase, including travel distance between letters.

The number of "clicks" a given word requires is equal to its length plus the number of navigation clicks it takes to get from each letter to the next. This second measurement depends greatly on the **layout** of letters that the app/device presents to the user. To that end, this repo has a number of word lists in `lists/usable/` sub-directory, each based on a different layout. The layouts are summarized below.

This project (the Rust code and the resulting word lists) is mostly a **proof of concept** to show how very specific word lists can aid users if we know the use-case.

Here's [a blog post](https://sts10.github.io//2022/10/24/a-good-netflix-password.html) I wrote explaining my inspiration and process of this project.

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

- This program assumes that if the user is, say, on the 'w' of a qwerty keyboard, they can't move left 3 times to reach the 'o'. (That'd be a neat optional feature though if you want to write a PR!) The program assumes that the user would have to move 7 to the right to get to the 'o'.

- The program doesn't calculate the distance between the last character of one word and the first character of the next word, since we can't know that until the passphrase is generated.

## Usage (How to generate word lists yourself)

This program currently creates "raw" lists of about 11,000 words. **Warning**: these raw lists are not uniquely decodable and may contain profane words.

To refine these "raw" lists into more usable word lists, I then used another program I wrote called [Tidy](https://github.com/sts10/tidy) and ran a command like this:

```bash
tidy -AAAA --whittle-to 7776 -KlL -m 3 -r ../reject-words/profane-words.txt -r ../reject-words/roman-numerals-lower.txt -r ../reject-words/britishisms.txt -r ../reject-words/repeated-letters.txt --samples --force -o lists/usable/long/alpha-line.txt lists/raw/alpha-line.txt
```

Among other things, this Tidy command uses an algorithm I wrote called [Schlinkert pruning](https://sts10.github.io/2022/08/12/efficiently-pruning-until-uniquely-decodable.html) to make the  lists **uniquely decodable**. Effectively this means that words from the resulting list can be safely combined without a delimiter (like `seasonreadilyrentallunarpioneerbolted`). 

You can find "usable", uniquely decodable lists in the `lists/usable` directory. 

### Generate raw word lists for 3 different layout types

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repo and `cd` into this directory.
3. `mkdir -p lists/raw && mkdir -p lists/usable/long && mkdir -p lists/usable/short`
4. `cargo run --release`

Locations of input file and created files are all hard-coded. Created lists will be printed to `lists/raw/`.

## Lists/layouts

Each list in the `lists/usable/short` and `lists/usable/long` sub-directories correspond to a different, common keyboard layout. "Long" lists are 7,776-words long, which means that each word adds approximately 12.925 bits to a passphrase. "Short" lists are 1,296-words long, which means that each word from them adds approximately 10.34 bits of entropy to a passphrase.

**Guidance**: To minimize the number of "clicks" you must execute to enter a passphrase, use the word list that corresponds to the keyboard layout you're asked to enter the passphrase with.

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

## Bits per click 

If you're deciding between the long and short lists, you can consult this table of **bits of entropy per click**. The table also has columns for "Medium (3,000 words)" and "Very Short (800 words)" lists that are NOT currently included in this repository. 

|             | Long (7776) | Medium (3000) | Short (1296) | Very Short (800) |
|-------------|-------------|---------------|--------------|------------------|
| QWERTY      | 0.534       | 0.6767        | 0.7747       | 0.8189           |
| AlphaSquare | 0.6         | 0.7561        | 0.8792       | 0.9319           |
| AlphaLine   | 0.2983      | 0.3987        | 0.4813       | 0.5242           |

Shorter word lists give a higher bits per click, which is efficient. But to make a passphrase with the same amount of total entropy you'd need to use more words from a short a list. One word from a 7,776-word list gives 12.925 bits of entropy, while one word from a 1,296-word list only give 10.34 bits. 

Thus, 4 words from a long, 7,776-word list gives 51.7 bits on entropy. 4 words from a short, 1,296-word list gives only 41.36 bits (coincidentally, adding a fifth word from a short list gets us to equivalent 51.7 bits). 

Lastly, I'll note that the above calculations do not include the number of clicks it takes to get from the last character of one word to the first word of the next. Thus it under-prices the cost of using a higher number of words (5 words vs. 4 words).

## Disclaimer: Profane words

These lists may have profane, indecent or otherwise objectionable words in them.

## Where the source words come from

The `source_list.txt` word list -- which is used by the Rust code to create the shorter, optimized lists -- came from Google Books Ngram word frequency data (and since edited). See below for more information.

## More word lists

You can find additional [word lists I've created here](https://github.com/sts10/generated-wordlists).

## On licensing/usage

This project and its word lists are licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/). See LICENSE file for more information.

The words for some of these generated word lists come from Google Books Ngram data. This data compilation "is licensed under a [Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/)" ([source](https://storage.googleapis.com/books/ngrams/books/datasetsv3.html)). This project has no association with Google, nor does Google endorse this project. [More information available at the original project's repo](https://github.com/sts10/common_word_list_maker).
