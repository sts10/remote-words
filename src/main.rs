use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
enum Layout {
    Qwerty,
    AlphaSquare,
    AlphaLine,
}

fn main() {
    let list_length = 11000;

    print_a_layout(Layout::Qwerty, "lists/raw/qwerty.txt", list_length);
    print_a_layout(
        Layout::AlphaSquare,
        "lists/raw/alpha-square.txt",
        list_length,
    );
    print_a_layout(Layout::AlphaLine, "lists/raw/alpha-line.txt", 11000);
}

fn print_a_layout(layout: Layout, output_filename: &str, list_length: usize) {
    let mut words_and_scores: Vec<(String, usize)> = vec![];
    let raw_word_list = make_vec_from_filenames(&[PathBuf::from("source_list.txt")]);
    for word in raw_word_list {
        words_and_scores.push((word.to_string(), number_of_clicks(word.to_string(), layout)));
    }
    words_and_scores.sort_by(|a, b| a.1.cmp(&b.1));

    let mut f = File::create(output_filename).expect("Unable to create file");
    for (i, word_and_score) in words_and_scores.iter().enumerate() {
        writeln!(f, "{}", word_and_score.0).expect("Unable to write word to file");
        if i >= list_length - 1 {
            break;
        }
    }
}

fn make_map(layout: Layout) -> HashMap<char, (usize, usize)> {
    match layout {
        Layout::Qwerty => HashMap::from([
            ('q', (0, 0)),
            ('w', (1, 0)),
            ('e', (2, 0)),
            ('r', (3, 0)),
            ('t', (4, 0)),
            ('y', (5, 0)),
            ('u', (6, 0)),
            ('i', (7, 0)),
            ('o', (8, 0)),
            ('p', (9, 0)),
            ('a', (0, 1)),
            ('s', (1, 1)),
            ('d', (2, 1)),
            ('f', (3, 1)),
            ('g', (4, 1)),
            ('h', (5, 1)),
            ('j', (6, 1)),
            ('k', (7, 1)),
            ('l', (8, 1)),
            ('z', (0, 2)),
            ('x', (1, 2)),
            ('c', (2, 2)),
            ('v', (3, 2)),
            ('b', (4, 2)),
            ('n', (5, 2)),
            ('m', (6, 2)),
        ]),
        Layout::AlphaSquare => HashMap::from([
            ('a', (0, 0)),
            ('b', (1, 0)),
            ('c', (2, 0)),
            ('d', (3, 0)),
            ('e', (4, 0)),
            ('f', (5, 0)),
            ('g', (0, 1)),
            ('h', (1, 1)),
            ('i', (2, 1)),
            ('j', (3, 1)),
            ('k', (4, 1)),
            ('l', (5, 1)),
            ('m', (0, 2)),
            ('n', (1, 2)),
            ('o', (2, 2)),
            ('p', (3, 2)),
            ('q', (4, 2)),
            ('r', (5, 2)),
            ('s', (0, 3)),
            ('t', (1, 3)),
            ('u', (2, 3)),
            ('v', (3, 3)),
            ('w', (4, 3)),
            ('x', (5, 3)),
            ('y', (0, 4)),
            ('z', (1, 4)),
        ]),
        Layout::AlphaLine => HashMap::from([
            ('a', (0, 0)),
            ('b', (1, 0)),
            ('c', (2, 0)),
            ('d', (3, 0)),
            ('e', (4, 0)),
            ('f', (5, 0)),
            ('g', (6, 0)),
            ('h', (7, 0)),
            ('i', (8, 0)),
            ('j', (9, 0)),
            ('k', (10, 0)),
            ('l', (11, 0)),
            ('m', (12, 0)),
            ('n', (13, 0)),
            ('o', (14, 0)),
            ('p', (15, 0)),
            ('q', (16, 0)),
            ('r', (17, 0)),
            ('s', (18, 0)),
            ('t', (19, 0)),
            ('u', (20, 0)),
            ('v', (21, 0)),
            ('w', (22, 0)),
            ('x', (23, 0)),
            ('y', (24, 0)),
            ('z', (25, 0)),
        ]),
    }
}

fn measure_distance_between_chars(char1: char, char2: char, layout: Layout) -> usize {
    let map = make_map(layout);
    let coordinates1 = map[&char1];
    let coordinates2 = map[&char2];
    ((coordinates1.0 as isize - coordinates2.0 as isize).abs()
        + (coordinates1.1 as isize - coordinates2.1 as isize).abs())
    .try_into()
    .unwrap()
}

fn measure_distance_of_word(word: String, layout: Layout) -> usize {
    let mut distance = 0;
    let char_vec: Vec<char> = word.chars().collect();
    for c in char_vec.windows(2) {
        distance += measure_distance_between_chars(c[0], c[1], layout);
    }
    distance
}

fn number_of_clicks(word: String, layout: Layout) -> usize {
    word.chars().count() + measure_distance_of_word(word, layout)
}
/// Takes a slice of `PathBuf`s representing the word list(s)
/// that the user has inputted to the program. Then iterates
/// through each file and addes each line to Vec<String>. (Blank
/// lines and duplicate links will be handled elsewhere.)
pub fn make_vec_from_filenames(filenames: &[PathBuf]) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    for filename in filenames {
        let f = match File::open(filename) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file {:?}: {}", filename, e),
        };
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!(
                        "Error reading a line from file {:?}: {}\nWill continue reading file.",
                        filename, e
                    );
                    continue;
                }
            };
            word_list.push(l);
        }
    }
    word_list
}
